use crate::model::storage::{AgentUpload, InsertableAgentUpload};
use crate::model::user::User;
use crate::{schema as s, view, DieselError};
use chrono::{DateTime, Utc};
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
        .set((
            s::agents::columns::uploaded.eq(true),
            s::agents::columns::uploaded_at.eq(Utc::now()),
        ))
        .get_result(conn)
}

pub fn mark_upload_as_failed(conn: &PgConnection, id: String) -> Result<AgentUpload, DieselError> {
    diesel::update(s::agents::dsl::agents.filter(s::agents::columns::id.eq(id)))
        .set(s::agents::columns::failed.eq(true))
        .get_result(conn)
}

pub fn mark_agent_as_deleted(conn: &PgConnection, id: String) -> Result<AgentUpload, DieselError> {
    diesel::update(s::agents::dsl::agents.filter(s::agents::columns::id.eq(id)))
        .set(s::agents::columns::deleted.eq(true))
        .get_result(conn)
}

pub fn list_agents(
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

pub fn get_agent_owner(conn: &PgConnection, agent_id: String) -> Result<User, DieselError> {
    s::agents::table
        .filter(s::agents::columns::id.eq(agent_id))
        .inner_join(s::users::table)
        .select(s::users::all_columns)
        .first(conn)
}

/// Like get_agent but it requires the existance of the agent otherwise it will result in an error.
pub fn get_agent_required(
    conn: &PgConnection,
    agent_id: String,
) -> Result<AgentUpload, DieselError> {
    s::agents::table
        .filter(s::agents::columns::id.eq(agent_id))
        .first(conn)
}

pub fn get_active_agent(
    conn: &PgConnection,
    user: i32,
    competition: i32,
) -> Result<Option<AgentUpload>, DieselError> {
    use view::active_agents::columns as c;
    view::active_agents::table
        .filter(c::competition.eq(competition))
        .filter(c::owner.eq(user))
        .first(conn)
        .optional()
}

/// Sets the active agent's active flag to false.
///
/// If there was no active agent for that user in that competition at the time of
/// this query `Ok(None)` is returned.
///
/// The return value is post update (i.e. active will always be false).
pub fn deactivate_agent(
    conn: &PgConnection,
    competition: i32,
    user: i32,
) -> Result<Option<AgentUpload>, DieselError> {
    use view::active_agents::columns as c;
    diesel::update(
        view::active_agents::table
            .filter(c::owner.eq(user))
            .filter(c::competition.eq(competition)),
    )
    .set(c::active.eq(false))
    .get_result(conn)
    .optional()
}

/// Sets the given agent's active flag to false if it is set to true.
///
/// If that agent did not exist or it was not active then `Ok(None)` is returned.
///
/// The return value is post update (i.e. active will always be false).
pub fn deactivate_agent_by_id(
    conn: &PgConnection,
    agent_id: String,
) -> Result<Option<AgentUpload>, DieselError> {
    use view::active_agents::columns as c;
    diesel::update(view::active_agents::table.filter(c::id.eq(agent_id)))
        .set(c::active.eq(false))
        .get_result(conn)
        .optional()
}

/// Sets the active field for this agent to true.
/// If another agent is currently active this will return an error.
///
/// If the agent does not exist this will return an error
pub fn activate_agent(conn: &PgConnection, agent_id: String) -> Result<AgentUpload, DieselError> {
    use s::agents::columns as c;
    diesel::update(s::agents::table.filter(c::id.eq(agent_id)))
        .set(c::active.eq(true))
        .get_result(conn)
}

pub fn get_active_agents_uploaded_before(
    conn: &PgConnection,
    competition: i32,
    before: DateTime<Utc>,
) -> Result<Vec<AgentUpload>, DieselError> {
    use view::active_agents::columns as c;
    view::active_agents::table
        .filter(c::competition.eq(competition))
        .filter(c::uploaded_at.lt(before))
        .get_results(conn)
}

// Returns agents were uploaded: true and deleted: false (this includes active agents).
// Just because an agent is uploaded but not active doesn't mean it is deletable (it
// may be in the queue for activation), therefore you should make sure the before is less than or
// equal to the upload time of a known active / in activation queue agent.
pub fn get_deletable_agents_uploaded_before(
    conn: &PgConnection,
    competition: i32,
    user: i32,
    before: DateTime<Utc>,
) -> Result<Vec<AgentUpload>, DieselError> {
    use s::agents::columns as c;
    s::agents::table
        .filter(c::competition.eq(competition))
        .filter(c::owner.eq(user))
        .filter(c::uploaded_at.lt(before))
        .get_results(conn)
}
