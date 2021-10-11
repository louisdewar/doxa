use crate::schema::{invites, users};
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};

/// The length in bytes for generating new invite IDs.
pub const INVITE_ID_LEN: usize = 20;

#[derive(Debug, Clone, Queryable)]
pub struct User {
    pub id: i32,
    pub admin: bool,
    pub username: String,
    pub password: String,
    pub token_generation: String,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub username: String,
    pub password: String,
    pub token_generation: String,
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "invites"]
pub struct Invite {
    pub id: String,
    pub username: Option<String>,
    pub enrollments: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

pub fn generate_invite_id() -> String {
    use rand::Rng;

    rand::thread_rng()
        .sample_iter(rand::distributions::Standard)
        .take(INVITE_ID_LEN)
        .map(|b: u8| format!("{:02x}", b))
        .collect()
}
