pub mod controller;
pub mod error;
pub mod extractor;
pub mod guard;
pub mod password;
pub mod route;
pub mod settings;
pub mod token;

pub use settings::Settings;

pub fn config(settings: Settings) -> impl FnOnce(&mut actix_web::web::ServiceConfig) {
    move |cfg| {
        cfg.data(settings);
        route::config(cfg);
    }
}
