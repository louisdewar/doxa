use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StartEvent {
    pub agents: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorEvent {
    pub error: String,
    pub debug: String,
}

// TODO: consider making _ERROR only occur for internal errors?

#[derive(Serialize, Deserialize)]
/// An agent had an error that was it's fault so it counts as a loss.
/// If there is also an error then this message `_FORFEIT` is sent before `_ERROR`
/// which in turn is sent before `_END`.
///
/// A forfeit does not necessarily mean the end of the game.
pub struct ForfeitEvent {
    pub agent_id: usize,
    pub stderr: Option<String>,
    // TODO: maybe an enum of reasons?
}
