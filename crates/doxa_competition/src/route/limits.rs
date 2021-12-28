use std::sync::Arc;

use doxa_auth::limiter::{GenericLimiter, Limiter};

pub struct CompetitionLimits {
    pub activations: Limiter,
}

impl CompetitionLimits {
    pub fn new(generic: Arc<GenericLimiter>) -> Self {
        CompetitionLimits {
            // This is tied to the upload limits
            activations: doxa_storage::limits::upload_attempts_limiter().build(&generic),
        }
    }
}
