pub mod controller;
pub mod error;
pub mod extractor;
pub mod guard;
pub mod limiter;
pub mod settings;

pub(crate) mod delegated;
pub(crate) mod route;
pub(crate) mod token;

use actix_web::web::Data;
use delegated::DelegatedAuthManager;
pub use doxa_core::autha_client::Client as AuthaClient;
pub use settings::Settings;

pub(crate) use doxa_core::autha_client::flow::User as AuthaUser;

pub fn config(settings: Settings) -> impl FnOnce(&mut actix_web::web::ServiceConfig) {
    move |cfg| {
        let delegated_auth = DelegatedAuthManager::new(settings.redis_db.clone());
        cfg.app_data(Data::new(delegated_auth));
        cfg.app_data(Data::new(settings));
        route::config(cfg);
    }
}
