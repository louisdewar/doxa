use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub(crate) enum ProviderFlow {
    Authenticated {
        /// Temporarily support the older auth_token (they are currently identical but auth_token) is being phased out)
        auth_token: String,
        refresh_token: String,
    },
    Incomplete {
        payload: Value,
    },
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub(crate) enum DelegatedAuthCheck {
    // Temporaily keep the old way while clients update
    Authenticated {
        auth_token: String,
        refresh_token: String,
    },
    Waiting,
}
