use diesel::{Insertable, Queryable};
use doxa_db::schema::users;

#[derive(Debug, Clone, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub username: String,
    pub password: String,
}
