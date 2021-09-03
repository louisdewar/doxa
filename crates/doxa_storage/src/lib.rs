// Maybe rename doxa_agent

mod controller;
mod error;
mod retrieval;
mod route;
mod settings;
mod storage;

use actix_web::web::Data;
pub use settings::Settings;
use storage::LocalStorage;

pub use reqwest::Error as RetrievalError;
pub use retrieval::AgentRetrieval;

pub fn config(settings: Settings) -> impl FnOnce(&mut actix_web::web::ServiceConfig) {
    move |cfg| {
        cfg.app_data(Data::new(LocalStorage::from_settings(&settings)));
        route::config(cfg);
    }
}
