use std::error;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

pub use crate::error::ForfeitError;
pub use crate::{context::GameContext, error::GameError};

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

    /// Runs the game until completion.
    /// TODO: allow this method to take in the competition
    async fn run<'a>(
        match_request: Self::MatchRequest,
        context: &mut GameContext<'a, Self>,
    ) -> Result<(), GameError<Self::Error>>;
}
