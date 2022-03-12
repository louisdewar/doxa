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
    pub execution_environment: String,
    pub file_size: i32,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "agents"]
pub struct InsertableAgentUpload {
    pub id: String,
    pub owner: i32,
    pub competition: i32,
    pub extension: String,
}

#[derive(AsChangeset)]
#[table_name = "agents"]
#[changeset_options(treat_none_as_null = "true")]
pub(crate) struct ActivateChangeset {
    pub active: bool,
    pub activated_at: Option<DateTime<Utc>>,
}
