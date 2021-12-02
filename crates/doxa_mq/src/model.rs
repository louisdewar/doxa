use doxa_core::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivationEvent {
    /// The id of the agent
    pub agent: String,
    /// If true the agent is activating otherwise deactivation
    pub activating: bool,
    /// The competition that this agent is part of
    pub competition: String,
}

#[derive(Serialize, Deserialize)]
pub struct MatchRequest<T> {
    /// The ids of the agents that are participating
    pub agents: Vec<String>,
    /// The competition specific payload
    pub payload: T,
    /// The id of the game
    pub game_id: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameEvent<T> {
    /// The timestamp when the event occured
    pub timestamp: DateTime<Utc>,
    /// System event types such as `_START` and `_END` begin with underscores, competition events
    /// must not
    pub event_type: String,
    /// Event ID within a particular game that is used for ordering.
    /// The ID must be unique within the game.
    pub event_id: u32,
    /// The id of the game
    pub game_id: i32,
    /// The event type specific payload
    pub payload: T,
}

impl<T> GameEvent<T> {
    pub fn try_map_payload<F: FnOnce(T) -> Result<New, Error>, New, Error>(
        self,
        f: F,
    ) -> Result<GameEvent<New>, Error> {
        Ok(GameEvent {
            payload: f(self.payload)?,
            timestamp: self.timestamp,
            event_type: self.event_type,
            event_id: self.event_id,
            game_id: self.game_id,
        })
    }
}

impl From<doxa_db::model::game::GameEvent> for GameEvent<doxa_db::serde_json::Value> {
    fn from(val: doxa_db::model::game::GameEvent) -> Self {
        GameEvent {
            timestamp: val.event_timestamp,
            event_type: val.event_type,
            event_id: val.event_id as u32,
            game_id: val.game,
            payload: val.payload,
        }
    }
}
