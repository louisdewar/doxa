use std::{path::PathBuf, sync::Arc};

use doxa_auth::limiter::GenericLimiter;

#[derive(Clone)]
pub struct Settings {
    pub root: PathBuf,
    pub generic_limiter: Arc<GenericLimiter>,
}
