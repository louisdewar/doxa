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
        .route("/auth/authorize", web::post().to(authorize))
        .route(
            "/auth/authorize_delegated",
            web::post().to(authorize_delegated),
        )
        .route("/auth/check_delegated", web::post().to(check_delegated));
}

async fn authorize(
    settings: web::Data<Settings>,
    request: web::Json<request::Authorize>,
) -> EndpointResult {
    let refresh_token = request.into_inner().refresh_token;
    Ok(HttpResponse::Ok().json(settings.autha_client.authorize(refresh_token).await??))
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
    delegated_auth: web::Data<DelegatedAuthManager>,
    body: web::Json<request::CheckDelegated>,
    settings: web::Data<Settings>,
) -> EndpointResult {
    let auth = delegated_auth
        .check_authenticated(&body.verification_code, &body.auth_secret)
        .await?;

    let response = if let Some(user_id) = auth {
        let refresh_token = settings
            .autha_client
            .issue_refresh_token(user_id)
            .await??
            .refresh_token;

        // TODO: use autha/jwt/issue
        response::DelegatedAuthCheck::Authenticated {
            auth_token: refresh_token.clone(),
            refresh_token,
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
        .authenticate(&body.verification_code, user.id_required()?)
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

    controller::handle_flow_response(db_pool, response).await
}

async fn verify_email(
    db_pool: web::Data<PgPool>,
    body: web::Json<request::VerifyEmail>,
    settings: web::Data<Settings>,
) -> EndpointResult {
    let autha = &settings.autha_client;
    let body = body.into_inner();

    let response = autha.verify_email(body).await??;

    controller::handle_flow_response(db_pool, response).await
}
