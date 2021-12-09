use crate::schema::{competitions, enrollment};

use diesel::{AsChangeset, Insertable, Queryable};

#[derive(Debug, Clone, Queryable)]
pub struct Competition {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[table_name = "competitions"]
pub struct InsertableCompetition {
    pub name: String,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "enrollment"]
pub struct Enrollment {
    pub user_id: i32,
    pub competition: i32,
}
