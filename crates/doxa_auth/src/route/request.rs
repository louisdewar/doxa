use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub(crate) struct Provider {
    pub provider_name: String,
    pub flow_name: String,
    pub payload: Value,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct VerifyEmail {
    pub verification_code: String,
}

#[derive(Deserialize)]
pub(crate) struct CheckDelegated {
    pub verification_code: String,
    pub auth_secret: String,
}

#[derive(Deserialize)]
pub(crate) struct AuthorizeDelegated {
    pub verification_code: String,
}
