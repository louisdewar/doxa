use crate::error::{AgentUploadError, FileTooLarge};
use crate::route::request::DownloadParams;
use crate::{
    error::{AgentGone, TooManyUploadAttempts},
    limits::UploadLimits,
};
use actix_files::NamedFile;
use actix_multipart::{Field, Multipart};
use actix_web::{web, HttpRequest, HttpResponse};
use doxa_auth::{error::CompetitionNotFound, guard::AuthGuard};
use doxa_core::tokio::fs::File;
use doxa_core::tokio::io::AsyncWriteExt;
use doxa_core::tracing::error;
use doxa_core::EndpointResult;
use doxa_db::PgPool;
use doxa_mq::MQPool;
use futures::{StreamExt, TryStreamExt};

mod request;
mod response;

use crate::{
    error::{
        AgentNotFound, CouldNotReadFile, CouldNotWriteFile, ExtensionMissing, FileMissing,
        InvalidExtension, UploadMultipartError,
    },
    LocalStorage,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/storage/upload/{competition}", web::post().to(upload));
    cfg.route(
        "/storage/download/{competition}/{agent}",
        web::get().to(download),
    );
}

async fn download(
    pool: web::Data<PgPool>,
    storage: web::Data<LocalStorage>,
    path: web::Path<(String, String)>,
    query: web::Query<DownloadParams>,
    req: HttpRequest,
) -> EndpointResult {
    let (competition_name, agent_id) = path.into_inner();

    let require_active = query.active;

    let competition = web::block({
        let pool = pool.clone();
        let conn = web::block(move || pool.get()).await??;
        move || doxa_db::action::competition::get_competition_by_name(&conn, &competition_name)
    })
    .await??
    .ok_or(CompetitionNotFound)?;

    let agent = web::block({
        let pool = pool.clone();
        let conn = web::block(move || pool.get()).await??;
        move || doxa_db::action::storage::get_agent(&conn, agent_id)
    })
    .await??
    .ok_or(AgentNotFound)?;

    if agent.competition != competition.id
        || !agent.uploaded
        || agent.failed
        || agent.deleted
        || (require_active && !agent.active)
    {
        return Err(AgentGone.into());
    }

    let file = storage
        .open_file(&competition.name, &agent.id)
        .await
        .map_err(CouldNotReadFile::from)?;

    let named_file = NamedFile::from_file(
        file.into_std().await,
        format!("{}.{}", agent.id, agent.extension),
    )
    .map_err(CouldNotReadFile::from)?;

    Ok(named_file.into_response(&req))
}

async fn process_field_upload(
    file: &mut File,
    mut field: Field,
    max_size: usize,
) -> Result<(), AgentUploadError> {
    let mut total = 0;
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(UploadMultipartError::from)?;
        total += data.len();

        if total > max_size {
            return Err(FileTooLarge.into());
        }

        file.write_all(&data)
            .await
            .map_err(CouldNotWriteFile::from)?;
    }

    Ok(())
}

async fn upload(
    pool: web::Data<PgPool>,
    mq_pool: web::Data<MQPool>,
    storage: web::Data<LocalStorage>,
    mut payload: Multipart,
    path: web::Path<String>,
    auth: AuthGuard<()>,
    limiter: web::Data<UploadLimits>,
) -> EndpointResult {
    let competition = path.into_inner();
    // Check if the user is enrolled
    let enrollment = web::block({
        let user_id = auth.id();
        let competition = competition.clone();
        let pool = pool.clone();
        let conn = web::block(move || pool.get()).await??;
        move || doxa_auth::controller::is_enrolled(&conn, user_id, competition)
    })
    .await??;

    let competition_id = enrollment.competition;

    if !auth.admin() {
        limiter
            .upload_attempts
            .get_permit(format!("{}-{}", competition, auth.id()))
            .await?
            .map_err(TooManyUploadAttempts::from)?;
    }

    let field = payload
        .try_next()
        .await
        .map_err(UploadMultipartError::from)?
        .ok_or(FileMissing)?;

    let content_disposition = field.content_disposition();
    let filename = content_disposition.get_filename().ok_or(ExtensionMissing)?;

    let (_, extension) = filename.split_once('.').ok_or(ExtensionMissing)?;
    let extension = extension.to_string();

    match extension.as_str() {
        "tar" | "tar.gz" => {}
        _ => {
            return Err(InvalidExtension { extension }.into());
        }
    }

    let (mut f, id) = storage
        .create_file(competition.clone())
        .await
        .map_err(CouldNotWriteFile::from)?;

    web::block({
        let user_id = auth.id();
        let pool = pool.clone();
        let id = id.clone();
        let conn = web::block(move || pool.get()).await??;
        move || {
            crate::controller::register_upload_start(
                &conn,
                id,
                user_id,
                competition_id,
                extension.to_string(),
            )
        }
    })
    .await??;

    // TODO: get max size from competition
    let max_size = 50_000_000;

    match process_field_upload(&mut f, field, max_size).await {
        Ok(()) => {}
        Err(e) => {
            web::block({
                let pool = pool.clone();
                let id = id.clone();
                let conn = web::block(move || pool.get()).await??;
                move || crate::controller::mark_upload_as_failed(&conn, id)
            })
            .await??;

            drop(f);
            if let Err(delete_error) = storage.delete_file(&competition, &id).await {
                error!(upload_error=%e, %delete_error, "error when deleting upload file during a failed upload");
            }

            return Err(e.into());
        }
    }

    let uploaded_agent = web::block({
        let pool = pool.clone();
        let conn = web::block(move || pool.get()).await??;
        let id = id.clone();
        move || crate::controller::mark_upload_as_complete(&conn, id)
    })
    .await??;

    let mq_conn = mq_pool.get().await?;
    doxa_mq::action::emit_activation_event(
        &mq_conn,
        &doxa_mq::model::ActivationEvent {
            competition: competition.clone(),
            agent: id.clone(),
            activating: true,
        },
    )
    .await?;

    if let Err(e) = crate::controller::delete_old_uploads(
        storage,
        pool,
        &competition,
        enrollment.competition,
        auth.id(),
        uploaded_agent.uploaded_at,
    )
    .await
    {
        error!(error=%e, "failed to delete old uploads");
    }

    Ok(HttpResponse::Ok().json(response::Upload { id, competition }))
}

// async fn cleanup(user: AuthGuard<Admin>) -> EndpointResult {}
