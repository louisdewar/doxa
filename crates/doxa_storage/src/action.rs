use doxa_db::{diesel, Insertable, Queryable};

use doxa_db::schema::users;

#[derive(Queryable, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub can_create_namespaces: bool,
}
