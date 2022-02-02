use std::sync::Arc;

use doxa_core::redis::RedisPool;

use crate::AuthaClient;

#[derive(Clone)]
pub struct Settings {
    pub allow_registration: bool,
    pub autha_client: Arc<AuthaClient>,
    pub redis_db: RedisPool,
    pub delegated_auth_url: url::Url,
    pub system_account_secret: String,
}
