use std::time::Duration;

use doxa_auth::limiter::{LimiterConfig, TokenBucket, ONE_DAY, ONE_HOUR};

/// This limiter is used by the upload system for every attempt.
/// It is also used to limit manual agent activations / reactivations.
pub fn default_upload_attempts_limiter(key: String) -> LimiterConfig {
    let mut limiter = LimiterConfig::new(key);

    limiter
        // 1 per minute
        .add_limit(TokenBucket::new(Duration::from_secs(60), 1))
        // 5 per 10 mins
        .add_limit(TokenBucket::new(Duration::from_secs(60 * 10), 5))
        // 20 per hour
        .add_limit(TokenBucket::new(ONE_HOUR, 20))
        // 80 per day
        .add_limit(TokenBucket::new(ONE_DAY, 80));

    limiter
}
