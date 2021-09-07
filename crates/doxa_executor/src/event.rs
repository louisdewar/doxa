use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StartEvent {
    pub agents: Vec<String>,
}
