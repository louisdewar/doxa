use doxa_core::chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub(crate) enum ProviderFlow {
    Authenticated { auth_token: String },
    Incomplete { payload: Value },
}
