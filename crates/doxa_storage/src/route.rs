use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use doxa_auth::guard::AuthGuard;
use doxa_core::{handle_doxa_error, EndpointResult};
use doxa_db::PgPool;
use futures::{StreamExt, TryStreamExt};
use tokio::io::AsyncWriteExt;

use crate::{
    error::{CouldNotWriteFile, FileMissing, UploadMultipartError},
    LocalStorage,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/storage/upload/{competition}", web::post().to(upload));
}

async fn upload(
    pool: web::Data<PgPool>,
    storage: web::Data<LocalStorage>,
    mut payload: Multipart,
    web::Path(competition): web::Path<String>,
    auth: AuthGuard<()>,
) -> EndpointResult {
    // Check if the user is enrolled
    let enrollment = handle_doxa_error!(
        web::block({
            let user_id = auth.user();
            let competition = competition.clone();
            let pool = pool.clone();
            let conn = handle_doxa_error!(web::block(move || { pool.get() }).await);
            move || doxa_auth::controller::is_enrolled(&conn, user_id, competition)
        })
        .await
    );

    let mut field = handle_doxa_error!(handle_doxa_error!(payload
        .try_next()
        .await
        .map_err(UploadMultipartError::from))
    .ok_or(FileMissing));

    let (mut f, id) = handle_doxa_error!(storage
        .create_file(competition)
        .await
        .map_err(CouldNotWriteFile::from));

    handle_doxa_error!(
        web::block({
            let user_id = auth.user();
            let pool = pool.clone();
            let id = id.clone();
            let conn = handle_doxa_error!(web::block(move || { pool.get() }).await);
            move || {
                crate::controller::register_upload_start(&conn, id, user_id, enrollment.competition)
            }
        })
        .await
    );

    // TODO: In future these kinds of errors should result in the file being cleaned up
    // and the database field updated indicating the error
    while let Some(chunk) = field.next().await {
        let data = handle_doxa_error!(chunk.map_err(|e| UploadMultipartError::from(e)));
        handle_doxa_error!(f
            .write_all(&data)
            .await
            .map_err(|e| CouldNotWriteFile::from(e)));
    }

    handle_doxa_error!(
        web::block({
            let conn = handle_doxa_error!(web::block(move || { pool.get() }).await);
            move || crate::controller::mark_upload_as_complete(&conn, id)
        })
        .await
    );

    Ok(HttpResponse::Ok().into())
}
