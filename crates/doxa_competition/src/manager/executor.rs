use std::{marker::PhantomData, sync::Arc};

use doxa_executor::{client::GameClient, error::GameManagerError, game::GameManager};

use doxa_core::{
    lapin::options::BasicAckOptions,
    tokio::{self, sync::Semaphore},
    tracing::{event, span, Instrument, Level},
};
use doxa_mq::model::MatchRequest;
use futures::future::FutureExt;
use futures::StreamExt;

use crate::Settings;

/// Listens for execution events and then spawns games.
pub struct ExecutionManager<C: GameClient> {
    client: PhantomData<C>,
    settings: Arc<Settings>,
    competition_name: &'static str,
    executor_permits: usize,
}

impl<C: GameClient> ExecutionManager<C> {
    pub(crate) fn new(
        settings: Arc<Settings>,
        competition_name: &'static str,
        executor_permits: usize,
    ) -> Self {
        assert!(executor_permits > 0);

        ExecutionManager {
            client: PhantomData,
            settings,
            competition_name,
            executor_permits,
        }
    }

    /// Spawns a task then listens for match request
    pub async fn start(self) {
        let connection = self
            .settings
            .mq_pool
            .get()
            .await
            .expect("Failed to get MQ connection");

        let mut consumer =
            doxa_mq::action::get_match_request_consumer(&connection, self.competition_name)
                .await
                .unwrap();

        let span = span!(
            Level::INFO,
            "execution event listener",
            competition = self.competition_name
        );

        tokio::spawn(
            async move {
                let executor_settings = self.settings.executor_settings.clone();
                let executor_limiter = Arc::new(Semaphore::new(self.executor_permits));

                while let Some(message) = consumer.next().await {
                    let permit = executor_limiter.clone().acquire_owned().await.unwrap();
                    // TODO: remove expects and convert to error logging
                    let (_, delivery) = message.expect("Error connecting to MQ");
                    let match_request: MatchRequest<C::MatchRequest> =
                        doxa_mq::action::deserialize(&delivery.data)
                            .expect("Improperly formatted message");
                    let game_id = match_request.game_id;

                    let span = span!(
                        Level::INFO,
                        "handle match request",
                        game_id = %game_id,
                        agents = ?match_request.agents
                    );

                    let event_channel =
                        doxa_mq::action::game_event_channel(&connection, self.competition_name)
                            .await
                            .unwrap();
                    let event_channel_name =
                        doxa_mq::action::game_event_queue_name(self.competition_name);

                    tokio::spawn({
                        let executor_settings = executor_settings.clone();
                        let competition_name = self.competition_name;
                        async move {
                            let game_manager = match GameManager::<C>::new(
                                executor_settings,
                                event_channel,
                                event_channel_name,
                                competition_name,
                                match_request,
                            )
                            .await
                            {
                                Ok(game_manger) => game_manger,
                                Err(error) => {
                                    // Between now and when the agent was first queued it is no
                                    // longer the correct active agent (e.g. because of error or
                                    // because a new one was uploads or because it was deleted).
                                    if let GameManagerError::StartAgent(doxa_executor::error::AgentError::AgentGone) = error {
                                        event!(Level::DEBUG, "not starting game because agent was gone");
                                    }
                                    // Should probably emit a game event instead of never ack-ing meaning infinite loop
                                    event!(Level::ERROR, %error, debug = ?error, "failed to start game manager");

                                    // Temporary always acknowledge on error to prevent infinite
                                    // loop, in future there should be a max retry count / max TTL along with some smart decisions about which errors are permanent and which aren't
                                    // Also look into using NACK instead of just doing nothing
                                    delivery
                                        .ack(BasicAckOptions::default())
                                        .await
                                        .expect("Failed to acknowledge MQ");
                                    return;
                                }
                            };

                            match game_manager.run().await {
                                Ok(()) => event!(Level::INFO, "game manager succesfully completed"),
                                Err(error) => {
                                    event!(Level::ERROR, %error, debug = ?error, "error running game manager")
                                }
                            }

                            delivery
                                .ack(BasicAckOptions::default())
                                .await
                                .expect("Failed to acknowledge MQ");
                        }
                        .then(|_| async move { drop(permit); })
                        .instrument(span)
                    });


                }
            }
            .instrument(span),
        );
    }
}
