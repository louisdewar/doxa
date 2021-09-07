use doxa_core::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UploadEvent {
    /// The name of the competition
    pub competition: String,
    /// The id of the agent
    pub agent: String,
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
