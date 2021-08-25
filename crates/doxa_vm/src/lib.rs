use serde::{Deserialize, Serialize};

pub mod error;
pub mod executor;
pub mod manager;

mod stream;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Language {
    #[serde(rename = "wasm")]
    WASM,
    #[serde(rename = "python")]
    Python,
}

#[derive(Serialize, Deserialize)]
pub struct ExecutionPlan {
    language: Language,
    entrypoint: String,
}

pub fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}
