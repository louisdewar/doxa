use doxa_core::chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Login {
    pub auth_token: String,
}

#[derive(Serialize)]
pub struct InviteInfo {
    pub username: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub enrollments: Vec<String>,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub username: String,
    pub admin: bool,
}
