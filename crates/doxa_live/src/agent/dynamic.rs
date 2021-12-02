use doxa_executor::agent::VMAgent;

use super::LiveAgent;

/// Represents either an AgentVM or a user over a websocket.
pub enum LiveDynamicAgent {
    Live(LiveAgent),
    VM(Box<VMAgent>),
}
