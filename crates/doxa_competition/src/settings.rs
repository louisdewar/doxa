use std::sync::Arc;

use doxa_auth::limiter::GenericLimiter;
use doxa_db::PgPool;
use doxa_executor::client::firecracker::FirecrackerBackendSettings;
pub use doxa_executor::HTTPClient;
use doxa_mq::MQPool;

pub struct Settings {
    pub executor_settings: Arc<doxa_executor::Settings>,
    pub firecracker_settings: FirecrackerBackendSettings,
    pub mq_pool: Arc<MQPool>,
    pub pg_pool: Arc<PgPool>,
    pub generic_limiter: Arc<GenericLimiter>,
    /// The base url to a competitions api such that appending `{competition_name}/_game/{game_id}/cancelled` yields the
    /// cancelled game endpoint.
    pub competitions_base_url: String,
    /// A client for making HTTP requests
    pub request_client: HTTPClient,
}
