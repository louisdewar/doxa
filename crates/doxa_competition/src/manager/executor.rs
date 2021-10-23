use std::{marker::PhantomData, sync::Arc};

use doxa_executor::{client::GameClient, game::GameManager};

use doxa_core::{
    lapin::options::BasicAckOptions,
    tokio,
    tracing::{event, span, Instrument, Level},
};
use doxa_mq::model::MatchRequest;
use futures::StreamExt;

use crate::Settings;

/// Listens for execution events and then spawns games.
pub struct ExecutionManager<C: GameClient> {
    client: PhantomData<C>,
    settings: Arc<Settings>,
    competition_name: &'static str,
}

impl<C: GameClient> ExecutionManager<C> {
    pub(crate) fn new(settings: Arc<Settings>, competition_name: &'static str) -> Self {
        ExecutionManager {
            client: PhantomData,
            settings,
            competition_name,
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

                while let Some(message) = consumer.next().await {
                    // TODO: remove expects and convert to error logging
                    let (_, delivery) = message.expect("Error connecting to MQ");
                    let match_request: MatchRequest<C::MatchRequest> =
                        doxa_mq::action::deserialize(&delivery.data)
                            .expect("Improperly formatted message");
                    let span = span!(
                        Level::INFO,
                        "handle match request",
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
                                    // TODO: What about an error since an agent is no longer active?
                                    // Should probably emit a game event instead of never ack-ing meaning infinite loop
                                    event!(Level::ERROR, %error, debug = ?error, "failed to start game manager");
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
                        .instrument(span)
                    }).await.unwrap(); // TODO: semaphore then don't need to await for each


                }
            }
            .instrument(span),
        );
    }

    // fn spawn_game(match_request: MatchRequest<C::MatchRequest>) {
    //     tokio::spawn(async move { C::start(match_request, context) });
    // }
}