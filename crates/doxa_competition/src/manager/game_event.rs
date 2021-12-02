use std::sync::Arc;

use doxa_core::{
    lapin::options::BasicAckOptions,
    tokio,
    tracing::{error, event, span, warn, Level},
    tracing_futures::Instrument,
};

use crate::Settings;
use doxa_mq::model::GameEvent;

use futures::StreamExt;

use crate::client::{Competition, Context};

pub(super) struct GameEventManager<C: Competition> {
    settings: Arc<Settings>,
    competition: Arc<C>,
    context: Arc<Context<C>>,
}

impl<C: Competition> GameEventManager<C> {
    pub fn new(settings: Arc<Settings>, competition: Arc<C>, context: Arc<Context<C>>) -> Self {
        GameEventManager {
            settings,
            competition,
            context,
        }
    }

    pub async fn start(self) {
        let connection = self
            .settings
            .mq_pool
            .get()
            .await
            .expect("Failed to get MQ connection");

        let mut consumer =
            doxa_mq::action::get_game_event_consumer(&connection, C::COMPETITION_NAME)
                .await
                .unwrap();
        let span = span!(
            Level::INFO,
            "game event listener",
            competition = C::COMPETITION_NAME
        );
        let future = async move {
            // NOTE for future self: for concurrency it's better to have multiple game event
            // managers than spawn a task per event.

            // TODO: better error handling with span

            // TODO: Since if there is an error it is possible for events to come in out of order
            // there should be a system whereby a game has a nullable field "last_handled_event"
            // and if this event is not `last_handled_event + 1` then we delay handling it
            // otherwise we check to see if there are any events sequentially after this one and
            // handle each in turn (including the current)
            while let Some(message) = consumer.next().await {
                // It might be easier for error handling if this was moved into it's own async fn
                let (_, delivery) = message.expect("Error getting message");

                let game_event: GameEvent<serde_json::Value> =
                    serde_json::from_slice(&delivery.data).expect("Improperly formatted message");
                event!(Level::INFO, %game_event.game_id, %game_event.event_type, "received game event for agent");

                let res = tokio::task::spawn_blocking({
                    let game_event = game_event.clone();
                    let pool = self.settings.pg_pool.clone();
                    move || {
                        let db = pool.get().unwrap();
                        doxa_db::action::game::add_event(
                            &db,
                            &doxa_db::model::game::GameEvent {
                                event_id: game_event.event_id as i32,
                                game: game_event.game_id,
                                event_timestamp: game_event.timestamp,
                                event_type: game_event.event_type,
                                payload: game_event.payload,
                            },
                        )
                    }
                })
                .await
                .unwrap();

                if let Err(error) = res {
                    if doxa_db::was_unique_key_violation(&error) {
                        warn!(?game_event, "already inserted game event into db, not inserting or notifying again as there was likely an error last time");
                        // TODO: decide whether to notify the event again.
                    } else {
                        error!(?game_event, "failed to insert event into db");
                        // This will not ACK
                        continue;
                    }
                } else {
                    let event_type = &game_event.event_type;

                    if event_type.starts_with('_') {
                        // System message, maybe do something with it? E.g. have on_game_start /
                        // on_game_end / on_game_error
                        //
                        // TODO: set game start / end / if error set end and error
                        match event_type.as_str() {
                            "_START" => {}

                            "_END" => {
                                tokio::task::spawn_blocking({
                                    let complete_time = game_event.timestamp;
                                    let game_id = game_event.game_id;
                                    let pool = self.settings.pg_pool.clone();
                                    move || {
                                        let db = pool.get().unwrap();
                                        doxa_db::action::game::set_game_complete_time(
                                            &db,
                                            game_id,
                                            complete_time,
                                        )
                                    }
                                })
                                .await
                                .unwrap()
                                .unwrap();
                            }
                            "_ERROR" => {}
                            "_FORFEIT" => {}
                            _ => {
                                error!(%event_type, ?game_event, "unknown event type");
                            }
                        }
                    } else {
                        let game_event = game_event
                            .try_map_payload(serde_json::from_value)
                            .expect("Improperly formatted client message");

                        if let Err(error) = self
                            .competition
                            .on_game_event(&self.context, game_event)
                            .await
                        {
                            event!(Level::ERROR, %error, debug = ?error, "on_game_event failed for agent");
                            // This will not ACK but right now this function will not be run again
                            // so it is somewhat pointless
                            continue;
                        }
                    }
                }

                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("Failed to acknowledge MQ");
            }
        };

        tokio::spawn(future.instrument(span));
    }
}
