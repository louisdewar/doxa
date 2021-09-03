use crate::schema::agents;

use diesel::{data_types::PgTimestamp, Insertable, Queryable};

#[derive(Debug, Clone, Queryable)]
pub struct AgentUpload {
    pub id: String,
    pub owner: i32,
    pub competition: i32,
    pub extension: String,
    pub uploaded_at: PgTimestamp,
    pub uploaded: bool,
    pub deleted: bool,
    pub failed: bool,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "agents"]
pub struct InsertableAgentUpload {
    pub id: String,
    pub owner: i32,
    pub competition: i32,
    pub extension: String,
}
