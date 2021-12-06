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
use limits::AuthLimits;
pub use settings::Settings;

pub fn config(settings: Settings) -> impl FnOnce(&mut actix_web::web::ServiceConfig) {
    move |cfg| {
        let limiter = AuthLimits::new(settings.generic_limiter.clone());

        cfg.app_data(Data::new(settings));
        cfg.app_data(Data::new(limiter));
        route::config(cfg);
    }
}
