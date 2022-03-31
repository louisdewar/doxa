use std::sync::Arc;

use doxa_competition::{
    client::{Competition, ForfeitError, GameClient},
    tokio::{self, sync::Semaphore},
    tracing::{event, info, span, Level},
};
use doxa_core::{lapin::options::BasicAckOptions, tracing_futures::Instrument};
use doxa_executor::{error::GameManagerError, game::GameManager};
use doxa_mq::model::MatchRequest;
use futures_util::{FutureExt, StreamExt};
use reqwest::Url;

pub use doxa_vm::backend::docker;

pub struct CompetitionManagerSettings {
    pub executor_permits: u32,
    pub api_base_url: Url,
    pub request_client: reqwest::Client,
    pub mq_pool: doxa_mq::MQPool,
    pub executor_settings: Arc<doxa_executor::Settings>,
    pub docker_settings: docker::DockerBackendSettings,
}

pub struct CompetitionNodeManager<C: Competition> {
    competition: Arc<C>,
    settings: CompetitionManagerSettings,
}

impl<C: Competition> CompetitionNodeManager<C> {
    pub fn new(competition: Arc<C>, settings: CompetitionManagerSettings) -> Self {
        CompetitionNodeManager {
            competition,
            settings,
        }
    }

    pub async fn start(self) {
        let competition_name = C::COMPETITION_NAME;
        let connection = self
            .settings
            .mq_pool
            .get()
            .await
            .expect("Failed to get MQ connection");

        let mut consumer =
            doxa_mq::action::get_match_request_consumer(&connection, competition_name, "gpu")
                .await
                .unwrap();

        info!(
            competition =%competition_name,
            "execution event listener",
        );

        let game_client = Arc::new(self.competition.build_game_client());

        //let executor_settings = self.settings.executor_settings.clone();
        let executor_limiter = Arc::new(Semaphore::new(self.settings.executor_permits as usize));

        while let Some(message) = consumer.next().await {
            let permit = executor_limiter.clone().acquire_owned().await.unwrap();
            // TODO: remove expects and convert to error logging
            let (_, delivery) = message.expect("Error connecting to MQ");
            let match_request: MatchRequest<
                <<C as Competition>::GameClient as GameClient>::MatchRequest,
            > = doxa_mq::action::deserialize(&delivery.data).expect("Improperly formatted message");
            let game_id = match_request.game_id;

            let span = span!(
                Level::INFO,
                "handle match request",
                game_id = %game_id,
                agents = ?match_request.agents,
                competition_name = %competition_name,
            );

            let event_channel = doxa_mq::action::game_event_channel(&connection, competition_name)
                .await
                .unwrap();
            let event_channel_name = doxa_mq::action::game_event_queue_name(competition_name);

            tokio::spawn({
                let mut cancel_endpoint = self.settings.api_base_url.clone();
                cancel_endpoint.path_segments_mut().unwrap().extend(&[
                    "competition".to_string(),
                    C::COMPETITION_NAME.to_string(),
                    "_game".to_string(),
                    format!("{}", game_id),
                    "cancelled".to_string(),
                ]);
                // let cancel_endpoint = format!(
                //     "{}{}/_game/{}/cancelled",
                //     self.settings.api_base_url, competition_name, game_id
                // );
                let request_client = self.settings.request_client.clone();
                let executor_settings = self.settings.executor_settings.clone();
                let competition_name = competition_name;
                let game_client = game_client.clone();
                let docker_settings = self.settings.docker_settings.clone();

                // let firecracker_settings = firecracker::FirecrackerBackendSettings {
                //     kernel_img: executor_settings.kernel_img.clone(),
                //     kernel_boot_args: executor_settings.kernel_boot_args.clone(),
                //     firecracker_path: executor_settings.firecracker_path.clone(),
                //     vcpus: 6,
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

                            let game_manager = match GameManager::<C::GameClient, docker::DockerBackend>::new(
                                executor_settings,
                                docker_settings,
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

                            match game_manager.run_with_cancel_check(cancel_endpoint.to_string(), request_client).await {
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
    }
}
