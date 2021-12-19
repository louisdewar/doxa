use std::io::ErrorKind;

use actix_web::web;
use diesel::PgConnection;
use doxa_core::{
    chrono::{DateTime, Utc},
    tracing::{debug, error, warn},
};
use doxa_db::{
    action,
    model::storage::{AgentUpload, InsertableAgentUpload},
    DieselError, PgPool,
};

use crate::{error::DeleteOldAgentsError, storage::LocalStorage};

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

/// Tries to delete old uploads, if there is a diesel error this will return with that error.
/// If there is a FileNotFound error when deleting the agent, we assume that it was already deleted
/// and simply continue (and mark it as deleted in the database).
/// If there is a different kind of IO error then the agent is NOT marked as deleted but this
/// will continue attempting to delete the other agent.
/// The error will be logged.
pub async fn delete_old_uploads(
    storage: web::Data<LocalStorage>,
    pool: web::Data<PgPool>,
    competition_name: &str,
    competition: i32,
    user: i32,
    before: DateTime<Utc>,
) -> Result<(), DeleteOldAgentsError> {
    let agents = web::block({
        let pool = pool.clone();
        let conn = web::block(move || pool.get()).await??;
        move || {
            action::storage::get_deletable_agents_uploaded_before(&conn, competition, user, before)
        }
    })
    .await??;

    debug!(n=%agents.len(), "deleting old agents");

    let mut total_bytes = 0;
    for agent in agents {
        match storage.delete_file(competition_name, &agent.id).await {
            Ok(size) => total_bytes += size,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                // This may be an issue but could legitmately happen e.g. if the last time this was
                // run, it was interrupted or if two uploads are happening simultaneously
                warn!(id=%agent.id, "file not found when deleting old agent");
            }
            Err(e) => {
                error!(error=%e, debug=?e, "IO error when deleting old agents");
                continue;
            }
        }

        web::block({
            let pool = pool.clone();
            let conn = web::block(move || pool.get()).await??;
            move || action::storage::mark_agent_as_deleted(&conn, agent.id)
        })
        .await??;
    }

    debug!(total_bytes=%total_bytes, "freed space from agents");

    Ok(())
}

// pub struct DeleteStatistics {
//     /// The number of bytes actually removed from the file system.
//     data_removed: u64,
//     /// The number of agents that did not exist in the file system.
//     /// This should be 0 unless this process was interrupted last time.
//     did_not_exist: u32,
// }
//
// /// Finds all the agents marked with uploaded: true AND deleted: false, that were uploaded before
// /// the currently active agent and deletes each from the disk in turn, setting deleted to false
// ///
// /// This will continue attempting to delete old agents as long as it gets records from the
// /// database.
// /// If, for example, it tried to delete an agent that did not exist, this will simply mark it as
// /// deleted in the database and continue (although it will not count it towards the delete total.
// /// Returns the number of bytes freed.
// pub async fn delete_old_uploads(pool: web::Data<PgPool>, user_id: i32) -> Result<u64, DieselError> {
//     let statistics = DeleteStatistics {
//         data_removed: 0,
//         did_not_exist: 0,
//     };
//
//
// }
