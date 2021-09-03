use std::{error, fmt};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{context::GameContext, error::GameError};

/// Maintains the game state, sending input to agents and handling the output.
#[async_trait]
pub trait GameClient: Send + Sync + 'static {
    type Error: error::Error + fmt::Debug + fmt::Display;
    /// The payload of the match request specific to this competition.
    /// This is wrapped in the system's own match request that includes information such as the
    /// agents participating. This is only for extra metadata for the competition, in many cases it
    /// may not be required and could be set to the unit type `()`.
    type MatchRequest: Serialize + DeserializeOwned + Send + 'static;

    /// Runs the game until completion.
    /// TODO: allow this method to take in the competition
    async fn run<'a>(
        match_request: Self::MatchRequest,
        context: &mut GameContext<'a>,
    ) -> Result<(), GameError<Self::Error>>;
}
