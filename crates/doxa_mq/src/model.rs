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
}

#[derive(Serialize, Deserialize)]
pub struct GameEvent<T> {
    /// The unix timestamp measured in seconds from the UNIX_EPOCH when the event occured
    timestamp: u64,
    /// System event types such as `_START` and `_END` begin with underscores, competition events
    /// must not
    event_type: String,
    /// Event ID within a particular game that is used for ordering.
    /// The ID must be unique within the game.
    event_id: u32,
    /// The event type specific payload
    payload: T,
}
