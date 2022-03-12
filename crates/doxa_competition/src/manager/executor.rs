use std::sync::Arc;

use doxa_executor::{
    client::{
        firecracker::{self, FirecrackerBackendSettings},
        ForfeitError, GameClient,
    },
    error::GameManagerError,
    game::GameManager,
};

use doxa_core::{
    lapin::options::BasicAckOptions,
    tokio::{self, sync::Semaphore},
    tracing::{event, info, span, Instrument, Level},
};
use doxa_mq::model::MatchRequest;
use futures::future::FutureExt;
use futures::StreamExt;

use crate::{client::Competition, Settings};

/// Listens for execution events and then spawns games.
pub struct ExecutionManager<C: Competition> {
    //firecracker_settings: Arc<FirecrackerBackendSettings>,
    settings: Arc<Settings>,
    executor_permits: usize,
    competition: Arc<C>,
}

impl<C: Competition> ExecutionManager<C> {
    pub(crate) fn new(
        //firecracker_settings: Arc<FirecrackerBackendSettings>,
        settings: Arc<Settings>,
        executor_permits: usize,
        competition: Arc<C>,
    ) -> Self {
        assert!(executor_permits > 0);

        ExecutionManager {
            //  firecracker_settings,
            settings,
            executor_permits,
            competition,
        }
    }

    /// Spawns a task then listens for match request
    pub async fn start(self) {
        let competition_name = C::COMPETITION_NAME;
        let connection = self
            .settings
            .mq_pool
            .get()
            .await
            .expect("Failed to get MQ connection");

        let mut consumer =
            doxa_mq::action::get_match_request_consumer(&connection, competition_name, "basic")
                .await
                .unwrap();

        info!(
            competition =%competition_name,
            "execution event listener",
        );

        let game_client = Arc::new(self.competition.build_game_client());

        tokio::spawn(async move {
            let executor_settings = self.settings.clone();
            let executor_limiter = Arc::new(Semaphore::new(self.executor_permits));

            while let Some(message) = consumer.next().await {
                let permit = executor_limiter.clone().acquire_owned().await.unwrap();
                // TODO: remove expects and convert to error logging
                let (_, delivery) = message.expect("Error connecting to MQ");
                let match_request: MatchRequest<
                    <<C as Competition>::GameClient as GameClient>::MatchRequest,
                > = doxa_mq::action::deserialize(&delivery.data)
                    .expect("Improperly formatted message");
                let game_id = match_request.game_id;

                let span = span!(
                    Level::INFO,
                    "handle match request",
                    game_id = %game_id,
                    agents = ?match_request.agents,
                    competition_name = %competition_name,
                );

                let event_channel =
                    doxa_mq::action::game_event_channel(&connection, competition_name)
                        .await
                        .unwrap();
                let event_channel_name = doxa_mq::action::game_event_queue_name(competition_name);

                tokio::spawn({
                    let cancel_endpoint = format!(
                        "{}{}/_game/{}/cancelled",
                        self.settings.competitions_base_url, competition_name, game_id
                    );
                    let request_client = self.settings.request_client.clone();
                    let executor_settings = self.settings.executor_settings.clone();
                    let competition_name = competition_name;
                    let game_client = game_client.clone();
                    let firecracker_settings = self.settings.firecracker_settings.clone();
                    // let firecracker_settings = firecracker::FirecrackerBackendSettings {
                    //     kernel_img: executor_settings.kernel_img.clone(),
                    //     kernel_boot_args: executor_settings.kernel_boot_args.clone(),
                    //     firecracker_path: executor_settings.firecracker_path.clone(),
                    //     vcpus: 6,
                    //     original_rootfs: executor_settings.rootfs.clone(),
                    // };
                    async move {
                            // In future there can be some smarter code in the event that the
                            // code below fails or the server unexpected shutsdown, we need to
                            // consider that some game events may have been emitted and processed.
                            // Perhaps it's more correct to only process game events at the end?
                            delivery
                                .ack(BasicAckOptions::default())
                                .await
                                .expect("Failed to acknowledge MQ");

                            let game_manager = match GameManager::<C::GameClient, firecracker::FirecrackerBackend>::new(
                                executor_settings,
                                firecracker_settings,
                                event_channel,
                                event_channel_name,
                                competition_name,
                                match_request,
                                game_client
                            )
                            .await
                            {
                                Ok(game_manger) => game_manger,
                                Err(error) => {
                                    // Between now and when the agent was first queued it is no
                                    // longer the correct active agent (e.g. because of error or
                                    // because a new one was uploads or because it was deleted).
                                    // This is not a problem, it's good that we don't run the game
                                    // in the case.
                                    if let GameManagerError::StartAgent(agent_error) = &error {
                                        let agent_error = &agent_error.source;
                                        if matches!(agent_error, &doxa_executor::error::AgentError::AgentGone) {
                                            event!(Level::DEBUG, "not starting game because agent was gone");
                                            return;
                                        }
                                    }
                                    // Should probably emit an error game event
                                    event!(Level::ERROR, %error, debug = ?error, "failed to start game manager");
                                    return;
                                }
                            };

                            info!("started game manager");

                            match game_manager.run_with_cancel_check(cancel_endpoint, request_client).await {
                                Ok(()) => event!(Level::INFO, "game manager successfully completed"),
                                Err(error) => {
                                    if error.forfeit().is_some() {
                                        event!(Level::INFO, forfeit=true, %error, debug = ?error, "error running game manager")
                                    } else {
                                        event!(Level::ERROR, forfeit=false, %error, debug = ?error, "error running game manager")
                                    }
                                }
                            }
                        }
                        .then(|_| async move { drop(permit); })
                        .instrument(span)
                });
            }
        });
    }
}
