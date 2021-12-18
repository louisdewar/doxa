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
    extension: String,
) -> Result<AgentUpload, DieselError> {
    action::storage::register_upload_start(
        conn,
        &InsertableAgentUpload {
            id,
            owner: user_id,
            competition,
            extension,
        },
    )
}

pub fn mark_upload_as_complete(
    conn: &PgConnection,
    id: String,
) -> Result<AgentUpload, DieselError> {
    action::storage::mark_upload_as_complete(conn, id)
}

pub fn mark_upload_as_failed(conn: &PgConnection, id: String) -> Result<AgentUpload, DieselError> {
    action::storage::mark_upload_as_failed(conn, id)
}

/// Finds all the agents marked with uploaded: true AND deleted: false, that were uploaded before
/// the currently active agent and deletes each from the disk in turn, setting deleted to false
pub async fn delete_old_uploads(pool: web::Data<PgPool>, user_id: i32) -> Result<()> {}
