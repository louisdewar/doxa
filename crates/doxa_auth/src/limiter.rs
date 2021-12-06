//! Generic rate limiting system using redis as a backend.

use std::{fmt::Display, sync::Arc, time::Duration};

use doxa_core::{
    redis::{
        redis::{self, AsyncCommands},
        RedisPool,
    },
    tracing::trace,
};

use crate::error::{GetLimiterPermitError, RateLimitReached};

pub const ONE_HOUR: Duration = Duration::from_secs(60 * 60);
pub const ONE_DAY: Duration = Duration::from_secs(60 * 60 * 24);

const REDIS_RATE_LIMIT_INCR_LUA: &str = r#"
    local current = redis.call('incr', KEYS[1])
    if current == 1 then
        redis.call('expire', KEYS[1], tonumber(ARGV[1]))
    end

    local ttl = redis.call('ttl', KEYS[1])

    return { current, ttl }
"#;

const REDIS_RATE_LIMIT_DECR_LUA: &str = r#"
    if redis.call('exists', KEYS[1]) == 1 then
        redis.call('decr', KEYS[1])
        -- TODO: consider also removing the key if value = 0
    end
"#;
/// A neat wrapper around a redis backend that helps with rate limiting various actions.
///
/// Generally it's better to use a `ConfiguredLimiter` (which wraps around this) as you don't need
/// to specify options for every method.
///
/// The rate limiters are designed to be approximately correct but block the large majority of
/// traffic over the limit (especially in consecutive windows).
pub struct GenericLimiter {
    redis_pool: RedisPool,
    inc_expire_script: redis::Script,
    undo_inc_script: redis::Script,
}

impl GenericLimiter {
    pub fn new(redis_pool: RedisPool) -> Self {
        let inc_expire_script = redis::Script::new(REDIS_RATE_LIMIT_INCR_LUA);
        let undo_inc_script = redis::Script::new(REDIS_RATE_LIMIT_DECR_LUA);

        GenericLimiter {
            redis_pool,
            inc_expire_script,
            undo_inc_script,
        }
    }

    /// Tries to acquire a permit to perform an action.
    /// The outer error represents errors that occured while checking/incrementing rate limits,
    /// this represents an internal server error.
    /// The inner error is if the user has hit rate limits.
    /// `Ok(Ok(()))` is the only response that represents the user is allowed to perform this
    /// action.
    pub async fn get_permit(
        &self,
        base_key: &str,
        limiters: &[TokenBucket],
    ) -> Result<Result<(), RateLimitReached>, GetLimiterPermitError> {
        // NOTE: if there are some performance issues (in the case that the user is blocked and we
        // loop though limiters just to find that out), we could create a key which is just the
        // `base_key` and once we detect an expiration we create this key with an expiration equal
        // to the amount of time the user has to wait until the last (i.e. releases a permit last)
        // limiter is available, that way we first check that one and skip the remaining checks
        // until that key expires.

        let mut redis = self.redis_pool.get().await?;

        // NOTE: in redis 7 (yet to be released at the time of writing), EXPIRE supports additional
        // options which greatly simplify this without the need of a script.
        //
        // Also expiretime is an option to use instead of TTL.

        for (limiter_id, limiter) in limiters.iter().enumerate() {
            // NOTE: the script returns the expiretime atomically here because it's possible for
            // the key to have expired between the time we detect there are no permits left and
            // then request the expiration time.
            let (current, ttl): (u32, u64) = self
                .inc_expire_script
                .prepare_invoke()
                .key(format!("{}-{}", base_key, limiter_id))
                .arg(limiter.duration.as_secs())
                .invoke_async(&mut redis)
                .await?;

            if current > limiter.permits {
                trace!(%base_key, %limiter_id, %current, "permits exceed limit");

                let end_limiter_id = limiter_id;

                // Loop through previous limiters to decrement them as this request didn't go
                // through.
                //
                // NOTE: there may have been an insertion, we will not undo that. This could lead
                // to a shorter time from the first successfull request (after this) until permits
                // are replenished, but this will not allow any kind of throughput advantage when
                // considering fixed windows. UPDATE: see the comment in the lua script, there is a
                // potential fix to this but I'd prefer to have some more comprehensive tests
                // first.
                for limiter_id in 0..end_limiter_id {
                    self.undo_inc_script
                        .prepare_invoke()
                        .key(format!("{}-{}", base_key, limiter_id))
                        .invoke_async(&mut redis)
                        .await?;
                }

                let mut max_ttl = ttl;
                // Find the maximum expiration time (we only need to check the expiration of the
                // buckets after this one as we know that none of the previous ones have
                // expired).
                for (limiter_id, limiter) in limiters.iter().enumerate().skip(end_limiter_id + 1) {
                    let ttl = redis.ttl(format!("{}-{}", base_key, limiter_id)).await?;

                    // Technically it's possible for the token to expire between these two calls so
                    // we check the option
                    if let Some(permits) = redis
                        .get::<_, Option<u32>>(format!("{}-{}", base_key, limiter_id))
                        .await?
                    {
                        if permits > limiter.permits && max_ttl < ttl {
                            max_ttl = ttl;
                        }
                    }
                }

                return Ok(Err(RateLimitReached { ttl: max_ttl }));
            }
        }

        Ok(Ok(()))
    }
}

/// Represents a rate limiter where a user can take n (n = `permits`) in a given timeframe
/// (`duration`) before the bucket resets.
pub struct TokenBucket {
    duration: Duration,
    permits: u32,
}

impl TokenBucket {
    pub fn new(duration: Duration, permits: u32) -> Self {
        TokenBucket { duration, permits }
    }
}

/// A limiter config is a collection of individual rate limits.
/// For example one config may limit to 10 actions in 10 minutes.
/// Another config may have two strategies (`TokenBucket`s) such that there is a limit of 10
/// actions in 10 minutes and 30 actions in 1 hour.
///
/// If no limits are specififed then the actions are unthrottled.
pub struct LimiterConfig {
    limits: Vec<TokenBucket>,
    limiter_id: String,
}

impl LimiterConfig {
    /// The limiter ID must be unique across all rate limiters across `doxa`.
    /// It is used to generic the keys.
    pub fn new(limiter_id: String) -> Self {
        LimiterConfig {
            limits: Vec::new(),
            limiter_id,
        }
    }

    pub fn add_limit(&mut self, limit: TokenBucket) -> &mut Self {
        self.limits.push(limit);

        self
    }

    pub fn build(self, generic: &Arc<GenericLimiter>) -> Limiter {
        Limiter::new(Arc::clone(generic), self)
    }
}

pub struct Limiter {
    generic: Arc<GenericLimiter>,
    config: LimiterConfig,
}

impl Limiter {
    pub fn new(generic: Arc<GenericLimiter>, config: LimiterConfig) -> Self {
        Limiter { generic, config }
    }

    /// Tries to acquire a permit to perform an action.
    /// `Ok(Ok(()))` is the only response that represents the user is allowed to perform this
    /// action.
    ///
    /// See [`GenericLimiter::get_permit`] for more information (this is internally called by this
    /// method).
    ///
    /// The key should uniquely identify an individual entity that you want to rate limit, e.g.
    /// `user_id` or `username`.
    ///
    /// It **does not** need to be unique between different rate limiters.
    ///
    /// The key is combined with this limiter's unique limiter_id.
    pub async fn get_permit<K: Display>(
        &self,
        key: K,
    ) -> Result<Result<(), RateLimitReached>, GetLimiterPermitError> {
        self.generic
            .get_permit(
                &format!("{}-{}", self.config.limiter_id, key),
                &self.config.limits,
            )
            .await
    }
}
