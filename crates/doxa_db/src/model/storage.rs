use crate::schema::agents;

use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};

#[derive(Debug, Clone, Queryable)]
pub struct AgentUpload {
    pub id: String,
    pub owner: i32,
    pub competition: i32,
    pub extension: String,
    pub uploaded_at: DateTime<Utc>,
    pub activated_at: Option<DateTime<Utc>>,
    pub uploaded: bool,
    pub deleted: bool,
    pub failed: bool,
    pub active: bool,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "agents"]
pub struct InsertableAgentUpload {
    pub id: String,
    pub owner: i32,
    pub competition: i32,
    pub extension: String,
}
