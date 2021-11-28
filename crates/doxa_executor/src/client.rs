use std::error;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

pub use crate::error::ForfeitError;
pub use crate::{context::GameContext, error::GameError};

pub const DEFAULT_AGENT_RAM_MB: usize = 128;

/// Maintains the game state, sending input to agents and handling the output.
#[async_trait]
pub trait GameClient: Send + Sync + 'static {
    type Error: error::Error + ForfeitError + Send + Sync + 'static;

    /// The payload of the match request specific to this competition.
    /// This is wrapped in the system's own match request that includes information such as the
    /// agents participating. This is only for extra metadata for the competition, in many cases it
    /// may not be required and could be set to the unit type `()`.
    type MatchRequest: Serialize + DeserializeOwned + Send + 'static;

    /// The game event that this system emits.
    /// Note: system events (ones beginning with `_` are not included here).
    /// These will be stored using JSON in the database.
    /// It is recommended for this to be an `enum` with `#[serde(tag = "type")]` or similar to make
    /// deserialization/storage simple.
    type GameEvent: Serialize + DeserializeOwned + Send + 'static;

    /// The amount of ram that an agent's VM is given measured in mega-bytes.
    /// This defaults to [`DEFAULT_AGENT_RAM_MB`].
    /// NOTE: this is the total amount of ram including that which is used by the guest OS not just
    /// the agent.
    const AGENT_RAM: usize = DEFAULT_AGENT_RAM_MB;

    /// Runs the game until completion.
    /// TODO: allow this method to take in the competition
    async fn run<'a>(
        match_request: Self::MatchRequest,
        context: &mut GameContext<'a, Self>,
    ) -> Result<(), GameError<Self::Error>>;
}
