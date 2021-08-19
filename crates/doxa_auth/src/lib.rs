pub mod controller;
pub mod error;
pub mod extractor;
pub mod guard;
pub mod password;
pub mod route;
pub mod settings;
pub mod token;

use actix_web::web::Data;
pub use settings::Settings;

pub fn config(settings: Settings) -> impl FnOnce(&mut actix_web::web::ServiceConfig) {
    move |cfg| {
        cfg.app_data(Data::new(settings));
        route::config(cfg);
    }
}
