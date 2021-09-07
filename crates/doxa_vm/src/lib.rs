use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod error;
pub mod executor;
pub mod manager;
pub mod recorder;

// TODO: make private again once there is a wrapper layer in manager
pub mod stream;

pub use manager::Manager;

pub use serde_yaml;

// TODO: maybe move to doxa_core? other crates may need it, e.g. for validation or locally running
#[derive(Serialize, Deserialize, Debug)]
pub enum Language {
    #[serde(rename = "wasm")]
    WASM,
    #[serde(rename = "python")]
    Python,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecutionConfig {
    pub language: Language,
    pub entrypoint: String,
    #[serde(default)]
    pub options: Options,
}

pub(crate) type Options = HashMap<String, serde_yaml::Value>;
