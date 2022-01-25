use actix_web::web;
use doxa_core::EndpointResult;
use doxa_db::PgPool;

use crate::{controller, Settings};

pub mod request;
pub(crate) mod response;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/provider_flow", web::post().to(provider_flow))
        .route("/auth/verify_email", web::post().to(verify_email));
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
