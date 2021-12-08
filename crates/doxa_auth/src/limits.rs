//! The specific limits for `doxa_auth`.

const LOGIN_ATTEMPT_LIMITER_ID: &str = "DOXA_LOGIN_ATTEMPT";

use std::{sync::Arc, time::Duration};

use crate::limiter::{GenericLimiter, Limiter, LimiterConfig, TokenBucket, ONE_DAY, ONE_HOUR};

pub struct AuthLimits {
    /// Rate limits login attempts for a particular user both successful and unsuccessful
    pub login_attempts: Limiter,
}

impl AuthLimits {
    pub fn new(generic: Arc<GenericLimiter>) -> Self {
        AuthLimits {
            login_attempts: login_attempts_limiter().build(&generic),
        }
    }
}

fn login_attempts_limiter() -> LimiterConfig {
    let mut limiter = LimiterConfig::new(LOGIN_ATTEMPT_LIMITER_ID.into());

    limiter
        // 5 per minute
        .add_limit(TokenBucket::new(Duration::from_secs(60), 5))
        // 30 per hour
        .add_limit(TokenBucket::new(ONE_HOUR, 30))
        // 80 per day
        .add_limit(TokenBucket::new(ONE_DAY, 30));

    limiter
}
