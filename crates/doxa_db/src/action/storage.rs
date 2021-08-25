use crate::model::storage::{AgentUpload, InsertableAgentUpload};
use crate::{schema as s, DieselError};
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};

pub fn register_upload_start(
    conn: &PgConnection,
    agent: &InsertableAgentUpload,
) -> Result<AgentUpload, DieselError> {
    diesel::insert_into(s::agents::table)
        .values(agent)
        .get_result(conn)
}

pub fn mark_upload_as_complete(
    conn: &PgConnection,
    id: String,
) -> Result<AgentUpload, DieselError> {
    diesel::update(s::agents::dsl::agents.filter(s::agents::columns::id.eq(id)))
        .set(s::agents::columns::uploaded.eq(true))
        .get_result(conn)
}

pub fn list_uploads(
    conn: &PgConnection,
    user: i32,
    competition: i32,
) -> Result<Vec<AgentUpload>, DieselError> {
    s::agents::table
        .filter(s::agents::columns::owner.eq(user))
        .filter(s::agents::columns::competition.eq(competition))
        .get_results(conn)
}

pub fn get_agent(
    conn: &PgConnection,
    agent_id: String,
) -> Result<Option<AgentUpload>, DieselError> {
    s::agents::table
        .filter(s::agents::columns::id.eq(agent_id))
        .first(conn)
        .optional()
}
