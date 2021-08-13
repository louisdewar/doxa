use actix_multipart::Multipart;
use actix_web::{put, web, HttpResponse};
use doxa_auth::guard::AuthGuard;
use doxa_core::{handle_doxa_error, EndpointResult};
use doxa_db::PgPool;
use futures::{StreamExt, TryStreamExt};
use tokio::io::AsyncWriteExt;

use crate::{
    error::{CouldNotWriteFile, FileMissing, UploadMultipartError},
    LocalStorage,
};

#[put("/storage/upload")]
async fn upload(
    pool: web::Data<PgPool>,
    storage: web::Data<LocalStorage>,
    mut payload: Multipart,
    auth: AuthGuard<()>,
) -> EndpointResult {
    let conn = handle_doxa_error!(web::block(move || { pool.get() }).await);

    let mut field = handle_doxa_error!(handle_doxa_error!(payload
        .try_next()
        .await
        .map_err(UploadMultipartError::from))
    .ok_or(FileMissing));

    let (mut f, id) = handle_doxa_error!(storage
        .create_file("temp_competition TODO".to_owned())
        .await
        .map_err(|e| CouldNotWriteFile::from(e)));

    // In future these kinds of errors should result in the file being cleaned up
    while let Some(chunk) = field.next().await {
        let data = handle_doxa_error!(chunk.map_err(|e| UploadMultipartError::from(e)));
        handle_doxa_error!(f
            .write_all(&data)
            .await
            .map_err(|e| CouldNotWriteFile::from(e)));
    }

    Ok(HttpResponse::Ok().into())
}
