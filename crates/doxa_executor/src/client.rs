use serde::{de::DeserializeOwned, Serialize};

/// Maintains the execution state, sending input to agents and handling the output
pub trait ExecutionClient {
    /// The type that is created when by the competition for initializing a match.
    /// This is then recieved by the execution client on match start.
    type MatchRequest: Serialize + DeserializeOwned;
    /// The type that is generated when a match is successfully completed and sent back to the
    /// competition for processing.
    ///
    /// TODO: consider changing name to support sending errors back to the main system and using a
    /// Result enum where the Err type has some common errors with a customisable one.
    type ExecutionResult: Serialize + DeserializeOwned;
}
