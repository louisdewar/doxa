use actix_web::{web, HttpResponse};
use doxa_core::EndpointResult;
use doxa_db::{action, serde_json, PgPool};

use crate::{
    controller,
    error::{InviteNotFound, RegistrationDisabled, TooManyLoginAttempts},
    limits::AuthLimits,
    Settings,
};

use self::response::InviteInfo;

mod request;
mod response;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/login", web::post().to(login))
        .route("/auth/register", web::post().to(register))
        .route(
            "/auth/invite/accept/{invite_id}",
            web::post().to(accept_invite),
        )
        .route("/auth/invite/info/{invite_id}", web::get().to(invite_info));
}

async fn login(
    db_pool: web::Data<PgPool>,
    body: web::Json<request::Login>,
    settings: web::Data<Settings>,
    limiter: web::Data<AuthLimits>,
) -> EndpointResult {
    let request::Login { username, password } = body.0;

    limiter
        .login_attempts
        .get_permit(&username)
        .await?
        .map_err(TooManyLoginAttempts::from)?;

    let token = web::block(move || {
        // In future this needs to be handled properly, perferably where .get is non-blocking
        // so the error can easily be handled by the macro (right either (1) a wrapper error type will
        // be needed or (2) the error needs to be incorporated into the output of create_user - (2) not
        // a nice solution but may be common enough that (1) will be doable with a generic
        // wrapper)
        let conn = db_pool.get().unwrap();
        controller::login(&conn, &settings.jwt_secret, &username, &password)
    })
    .await??;

    Ok(HttpResponse::Ok().json(response::Login { auth_token: token }))
}

async fn register(
    db_pool: web::Data<PgPool>,
    body: web::Json<request::Register>,
    settings: web::Data<Settings>,
) -> EndpointResult {
    if !settings.allow_registration {
        return Err(RegistrationDisabled.into());
    }

    web::block(move || {
        let request::Register { username, password } = body.0;
        // In future this needs to be handled properly
        let conn = db_pool.get().unwrap();
        controller::create_user(&conn, username, &password)
    })
    .await??;

    Ok(HttpResponse::Ok().json(serde_json::json!({})))
}

async fn accept_invite(
    db_pool: web::Data<PgPool>,
    body: web::Json<request::Register>,
    invite_id: web::Path<String>,
) -> EndpointResult {
    let invite_id = invite_id.into_inner();
    web::block(move || {
        let request::Register { username, password } = body.0;
        // In future this needs to be handled properly
        let conn = db_pool.get().unwrap();
        controller::accept_invite(&conn, invite_id, username, &password)
    })
    .await??;

    Ok(HttpResponse::Ok().json(serde_json::json!({})))
}

async fn invite_info(db_pool: web::Data<PgPool>, invite_id: web::Path<String>) -> EndpointResult {
    let invite_id = invite_id.into_inner();

    let invite = web::block(move || {
        // In future this needs to be handled properly
        let conn = db_pool.get().unwrap();
        action::user::get_invite(&conn, invite_id)
    })
    .await??
    .ok_or(InviteNotFound)?;

    Ok(HttpResponse::Ok().json(InviteInfo {
        username: invite.username,
        expires_at: invite.expires_at,
        enrollments: invite.enrollments,
    }))
}
