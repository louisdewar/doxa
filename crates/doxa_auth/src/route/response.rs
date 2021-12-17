use doxa_core::chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Login {
    pub auth_token: String,
}

#[derive(Serialize)]
pub(crate) struct InviteInfo {
    pub username: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub enrollments: Vec<String>,
}
