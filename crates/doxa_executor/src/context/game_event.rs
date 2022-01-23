use std::marker::PhantomData;

use doxa_core::{
    chrono::Utc,
    lapin::{self, Channel},
};
use doxa_mq::model::GameEvent;
use serde::Serialize;

use crate::{
    client::GameClient,
    event::{CancelledEvent, ErrorEvent, StartEvent},
};

use std::error::Error;

pub(crate) struct GameEventContext<C: GameClient + ?Sized> {
    game_id: i32,
    event_id: u32,
    event_queue_name: String,
    event_channel: Channel,
    client: PhantomData<C>,
}

impl<'a, C: GameClient> GameEventContext<C> {
    pub fn new(event_channel: Channel, event_queue_name: String, game_id: i32) -> Self {
        GameEventContext {
            game_id,
            event_id: 0,
            event_channel,
            event_queue_name,
            client: PhantomData::default(),
        }
    }

    pub(crate) async fn emit_start_event(
        &mut self,
        agents: Vec<String>,
    ) -> Result<(), lapin::Error> {
        self.emit_event_raw(StartEvent { agents }, "_START".to_string())
            .await
    }

    pub(crate) async fn emit_cancelled_event(&mut self) -> Result<(), lapin::Error> {
        self.emit_event_raw(CancelledEvent {}, "_CANCELLED".to_string())
            .await
    }
    pub(crate) async fn emit_end_event(&mut self) -> Result<(), lapin::Error> {
        // TODO: end event data, e.g. total time spent, maybe whether it completed succesfully or
        // not
        self.emit_event_raw((), "_END".to_string()).await
    }

    pub(crate) async fn emit_error_event<E: Error>(
        &mut self,
        error: &E,
        vm_logs: Vec<Option<String>>,
    ) -> Result<(), lapin::Error> {
        self.emit_event_raw(
            ErrorEvent {
                error: format!("{}", error),
                debug: format!("{:?}", error),
                vm_logs,
            },
            "_ERROR".to_string(),
        )
        .await
    }

    pub(crate) async fn emit_event_raw<T: Serialize>(
        &mut self,
        payload: T,
        event_type: String,
    ) -> Result<(), lapin::Error> {
        let timestamp = Utc::now();
        let game_event = GameEvent {
            event_id: self.event_id,
            timestamp,
            event_type,
            payload,
            game_id: self.game_id,
        };
        self.event_id += 1;

        doxa_mq::action::publish(
            &self.event_channel,
            &self.event_queue_name,
            serde_json::to_vec(&game_event).unwrap(),
        )
        .await
        .map(|_| ())
    }
}
