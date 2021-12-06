use std::{sync::Arc, time::Duration};

use doxa_auth::limiter::{GenericLimiter, Limiter, LimiterConfig, TokenBucket, ONE_DAY, ONE_HOUR};

const AGENT_UPLOAD_ATTEMPT_LIMITER_ID: &str = "DOXA_AGENT_UPLOAD_ATTEMPT";

pub struct UploadLimits {
    pub upload_attempts: Limiter,
}

impl UploadLimits {
    pub fn new(generic: Arc<GenericLimiter>) -> Self {
        UploadLimits {
            upload_attempts: upload_attempts_limiter().build(&generic),
        }
    }
}

// TODO: make it so that each competition can specify their own limits (probably will need some
// kind of hashmap of competiton name => limiter)
fn upload_attempts_limiter() -> LimiterConfig {
    let mut limiter = LimiterConfig::new(AGENT_UPLOAD_ATTEMPT_LIMITER_ID.into());

    limiter
        // 1 per minute
        .add_limit(TokenBucket::new(Duration::from_secs(60), 1))
        // 10 per hour
        .add_limit(TokenBucket::new(ONE_HOUR, 10))
        // 40 per day
        .add_limit(TokenBucket::new(ONE_DAY, 40));

    limiter
}
