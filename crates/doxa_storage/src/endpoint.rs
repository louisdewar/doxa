use actix_multipart::Multipart;
use actix_web::{error::BlockingError, put, web, HttpResponse};
use doxa_core::{handle_doxa_error, EndpointResult};
use doxa_db::PgPool;

use futures::{StreamExt, TryStreamExt};

use crate::error::{CouldNotWriteFile, UploadMultipartError};

#[put("/n/{namespace}/{project}/upload")]
async fn upload(
    pool: web::Data<PgPool>,
    storage: web::Data<crate::LocalStorage>,
    mut payload: Multipart,
    web::Path((namespace, project)): web::Path<(String, String)>,
) -> EndpointResult {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let mut f = handle_doxa_error!(storage.create_temp().await.map_err(|e| {
            match e {
                BlockingError::Error(e) => BlockingError::Error(CouldNotWriteFile::from(e)),
                BlockingError::Canceled => BlockingError::Canceled,
            }
        }));

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = handle_doxa_error!(chunk.map_err(|e| UploadMultipartError::from(e)));
            handle_doxa_error!(f
                .write_all(&data)
                .await
                .map_err(|e| CouldNotWriteFile::from(e)));
        }
    }

    Ok(HttpResponse::Ok().into())
}
