use diesel::PgConnection;
use doxa_db::{
    action,
    model::storage::{AgentUpload, InsertableAgentUpload},
    DieselError,
};

use crate::error::AgentUploadError;

// use actix_multipart::Multipart;
// use diesel::PgConnection;
// use futures::{StreamExt, TryStreamExt};
// use tokio::io::AsyncWriteExt;
//
// use crate::error::{AgentUploadError, CouldNotWriteFile, UploadMultipartError};
//
// pub async fn upload_agent(
//     conn: &PgConnection,
//     storage: &crate::LocalStorage,
//     mut payload: Multipart,
// ) -> Result<(), AgentUploadError> {
//     while let Ok(Some(mut field)) = payload.try_next().await {
//         let (mut f, id) = storage
//             .create_file("temp_competition TODO".to_owned())
//             .await
//             .map_err(|e| CouldNotWriteFile::from(e))?;
//
//         while let Some(chunk) = field.next().await {
//             let data = chunk.map_err(|e| UploadMultipartError::from(e))?;
//             f.write_all(&data)
//                 .await
//                 .map_err(|e| CouldNotWriteFile::from(e))?;
//         }
//     }
//
//     Ok(())
// }
//

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
