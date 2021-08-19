use diesel::PgConnection;
use doxa_db::{
    action,
    model::storage::{AgentUpload, InsertableAgentUpload},
    DieselError,
};

pub fn register_upload_start(
    conn: &PgConnection,
    id: String,
    user_id: i32,
    competition: i32,
) -> Result<AgentUpload, DieselError> {
    action::storage::register_upload_start(
        conn,
        &InsertableAgentUpload {
            id,
            owner: user_id,
            competition,
        },
    )
}

pub fn mark_upload_as_complete(
    conn: &PgConnection,
    id: String,
) -> Result<AgentUpload, DieselError> {
    action::storage::mark_upload_as_complete(conn, id)
}
