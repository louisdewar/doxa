use std::sync::Arc;

use doxa_auth::limiter::{GenericLimiter, Limiter, LimiterConfig};

pub struct CompetitionLimits {
    pub activations: Limiter,
}

impl CompetitionLimits {
    pub fn new(generic: Arc<GenericLimiter>, activations: LimiterConfig) -> Self {
        CompetitionLimits {
            // This is tied to the upload limits
            activations: activations.build(&generic),
        }
    }
}
