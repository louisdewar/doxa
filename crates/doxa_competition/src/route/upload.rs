use doxa_auth::guard::AuthGuard;
use doxa_core::{actix_web::web, EndpointResult};
use doxa_db::PgPool;
use doxa_mq::MQPool;
use doxa_storage::{LocalStorage, Multipart};

use crate::client::Competition;

use super::limits::CompetitionLimits;

/// The default route for `_upload`.
pub async fn upload<C: Competition + ?Sized>(
    pool: web::Data<PgPool>,
    mq_pool: web::Data<MQPool>,
    storage: web::Data<LocalStorage>,
    payload: Multipart,
    auth: AuthGuard<()>,
    limits: web::Data<CompetitionLimits>,
) -> EndpointResult {
    doxa_storage::route::upload(
        pool,
        mq_pool,
        storage,
        payload,
        C::COMPETITION_NAME.to_string(),
        auth,
        &limits.activations,
    )
    .await
}
