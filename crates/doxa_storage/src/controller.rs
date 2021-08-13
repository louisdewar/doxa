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
