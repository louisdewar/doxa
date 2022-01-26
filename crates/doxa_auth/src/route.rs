use actix_web::{web, HttpResponse};
use doxa_core::EndpointResult;
use doxa_db::PgPool;

use crate::{controller, delegated::DelegatedAuthManager, guard::AuthGuard, Settings};

pub mod request;
pub(crate) mod response;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/provider_flow", web::post().to(provider_flow))
        .route("/auth/verify_email", web::post().to(verify_email))
        .route("/auth/start_delegated", web::post().to(start_delegated))
        .route(
            "/auth/authorize_delegated",
            web::post().to(authorize_delegated),
        )
        .route("/auth/check_delegated", web::post().to(check_delegated));
}

async fn start_delegated(
    settings: web::Data<Settings>,
    delegated_auth: web::Data<DelegatedAuthManager>,
) -> EndpointResult {
    let auth = delegated_auth
        .create(settings.delegated_auth_url.clone())
        .await?;

    Ok(HttpResponse::Ok().json(auth))
}

async fn check_delegated(
    db_pool: web::Data<PgPool>,
    delegated_auth: web::Data<DelegatedAuthManager>,
    body: web::Json<request::CheckDelegated>,
    settings: web::Data<Settings>,
) -> EndpointResult {
    let auth = delegated_auth
        .check_authenticated(&body.verification_code, &body.auth_secret)
        .await?;

    let response = if let Some(user_id) = auth {
        let user = web::block(move || {
            let conn = db_pool.get().unwrap();
            doxa_db::action::user::get_user_by_id(&conn, user_id)
        })
        .await??;

        response::DelegatedAuthCheck::Authenticated {
            auth_token: controller::generate_new_jwt_token(&user, &settings.jwt_secret),
        }
    } else {
        response::DelegatedAuthCheck::Waiting
    };

    Ok(HttpResponse::Ok().json(response))
}

async fn authorize_delegated(
    delegated_auth: web::Data<DelegatedAuthManager>,
    body: web::Json<request::AuthorizeDelegated>,
    user: AuthGuard,
) -> EndpointResult {
    delegated_auth
        .authenticate(&body.verification_code, user.id())
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({})))
}

async fn provider_flow(
    db_pool: web::Data<PgPool>,
    body: web::Json<request::Provider>,
    settings: web::Data<Settings>,
) -> EndpointResult {
    let autha = &settings.autha_client;
    let body = body.into_inner();

    let response = autha
        .provider_flow(&body.provider_name, &body.flow_name, body.payload)
        .await??;

    controller::handle_flow_response(&settings, db_pool, response).await
}

async fn verify_email(
    db_pool: web::Data<PgPool>,
    body: web::Json<request::VerifyEmail>,
    settings: web::Data<Settings>,
) -> EndpointResult {
    let autha = &settings.autha_client;
    let body = body.into_inner();

    let response = autha.verify_email(body).await??;

    controller::handle_flow_response(&settings, db_pool, response).await
}
