use crate::model::storage::{AgentUpload, InsertableAgentUpload};
use crate::{schema as s, DieselError};
use diesel::query_dsl::methods::OrderDsl;
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

pub fn get_active_agent(
    conn: &PgConnection,
    user: i32,
    competition: i32,
) -> Result<Option<AgentUpload>, DieselError> {
    use s::agents::columns as c;
    s::agents::table
        .filter(c::owner.eq(user))
        .filter(c::competition.eq(competition))
        .filter(c::competition.eq(competition))
        .filter(c::failed.eq(false))
        .filter(c::uploaded.eq(true))
        .filter(c::deleted.eq(false))
        .order_by(c::uploaded_at.desc())
        .first(conn)
        .optional()
}
