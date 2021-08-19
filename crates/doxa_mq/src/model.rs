use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UploadEvent {
    /// The name of the competition
    pub competition: String,
    /// The id of the agent
    pub agent: String,
}
