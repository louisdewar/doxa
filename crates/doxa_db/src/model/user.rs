use crate::schema::users;
use diesel::{Insertable, Queryable};

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
