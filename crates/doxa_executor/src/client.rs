use std::error;

use async_trait::async_trait;
pub use doxa_vm::mount::Mount;
use serde::{de::DeserializeOwned, Serialize};

pub use crate::error::ForfeitError;
pub use crate::{context::GameContext, error::GameError};

pub const DEFAULT_AGENT_RAM_MB: u64 = 256;
pub const DEFAULT_AGENT_SCRATCH_MB: u64 = 256;

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
    const AGENT_RAM_MB: u64 = DEFAULT_AGENT_RAM_MB;

    /// The amount of scratch space that an agent's VM is given measured in mega-bytes.
    /// This defaults to [`DEFAULT_AGENT_SCRATCH_MB`].
    ///
    /// Scratch space is mounted at /scratch and is used to store agent files while they download
    /// among other uses.
    const AGENT_SCRATCH_MB: u64 = DEFAULT_AGENT_SCRATCH_MB;

    /// An optional list of additional mounts for the VM (defaults to empty vec)
    fn additional_mounts(&self, _match_request: &Self::MatchRequest) -> Vec<Mount> {
        vec![]
    }

    /// Runs the game until completion.
    async fn run<'a>(
        &self,
        match_request: Self::MatchRequest,
        context: &mut GameContext<'a, Self>,
    ) -> Result<(), GameError<Self::Error>>;
}
