use actix_web::{web, HttpResponse};
use doxa_core::EndpointResult;
use doxa_db::{serde_json, PgPool};

use crate::{controller, Settings};

mod request;
mod response;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/login", web::post().to(login))
        .route("/auth/register", web::post().to(register));
}

async fn login(
    db_pool: web::Data<PgPool>,
    body: web::Json<request::Login>,
    settings: web::Data<Settings>,
) -> EndpointResult {
    let token = web::block(move || {
        let request::Login { username, password } = body.0;
        // In future this needs to be handled properly, perferably where .get is non-blocking
        // so the error can easily be handled by the macro (right either (1) a wrapper error type will
        // be needed or (2) the error needs to be incorporated into the output of create_user - (2) not
        // a nice solution but may be common enough that (1) will be doable with a generic
        // wrapper)
        let conn = db_pool.get().unwrap();
        controller::login(&conn, &settings.jwt_secret, &username, &password)
    })
    .await??;

    Ok(HttpResponse::Ok()
        .json(response::Login { auth_token: token })
        .into())
}

async fn register(
    db_pool: web::Data<PgPool>,
    body: web::Json<request::Register>,
) -> EndpointResult {
    web::block(move || {
        let request::Register { username, password } = body.0;
        // In future this needs to be handled properly
        let conn = db_pool.get().unwrap();
        controller::create_user(&conn, username, &password)
    })
    .await??;

    Ok(HttpResponse::Ok().json(serde_json::json!({})).into())
}
