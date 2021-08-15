// Maybe rename doxa_agent

mod controller;
mod error;
mod route;
mod settings;
mod storage;

pub use settings::Settings;
use storage::LocalStorage;

pub fn config(settings: Settings) -> impl FnOnce(&mut actix_web::web::ServiceConfig) {
    move |cfg| {
        cfg.data(LocalStorage::from_settings(&settings));
        cfg.data(settings);
        route::config(cfg);
    }
}
