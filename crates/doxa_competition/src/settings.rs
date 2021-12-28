use std::sync::Arc;

use doxa_auth::limiter::GenericLimiter;
use doxa_db::PgPool;
use doxa_mq::MQPool;

pub struct Settings {
    pub executor_settings: Arc<doxa_executor::Settings>,
    pub mq_pool: Arc<MQPool>,
    pub pg_pool: Arc<PgPool>,
    pub generic_limiter: Arc<GenericLimiter>,
}
