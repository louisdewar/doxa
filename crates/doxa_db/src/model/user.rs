use crate::schema::users;
use diesel::{Insertable, Queryable};
use serde_json::Value;

/// The length in bytes for generating new invite IDs.
pub const INVITE_ID_LEN: usize = 20;

#[derive(Debug, Clone, Queryable)]
pub struct User {
    pub id: i32,
    pub admin: bool,
    pub username: String,
    pub token_generation: String,
    pub extra: Value,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub id: i32,
    pub username: String,
    pub token_generation: String,
    pub extra: Value,
}
