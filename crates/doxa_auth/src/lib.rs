pub mod controller;
pub mod error;
pub mod extractor;
pub mod guard;
pub mod limiter;
pub mod route;
pub mod settings;
pub mod token;

mod limits;
mod password;

use actix_web::web::Data;
pub use doxa_core::autha_client::Client as AuthaClient;
use limits::AuthLimits;
pub use settings::Settings;

pub(crate) use doxa_core::autha_client::flow::User as AuthaUser;

pub fn config(settings: Settings) -> impl FnOnce(&mut actix_web::web::ServiceConfig) {
    move |cfg| {
        let limiter = AuthLimits::new(settings.generic_limiter.clone());

        cfg.app_data(Data::new(settings));
        cfg.app_data(Data::new(limiter));
        route::config(cfg);
    }
}
