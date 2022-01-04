// Maybe rename doxa_agent

mod controller;
mod error;
pub mod limits;
mod retrieval;
pub mod route;
mod settings;
mod storage;

pub use actix_multipart::Multipart;
use actix_web::web::Data;
pub use settings::Settings;
pub use storage::LocalStorage;

pub use reqwest::Error as RetrievalError;
pub use retrieval::AgentRetrieval;

pub fn config(settings: Settings) -> impl FnOnce(&mut actix_web::web::ServiceConfig) {
    move |cfg| {
        cfg.app_data(Data::new(LocalStorage::new(settings.root)));
        route::config(cfg);
    }
}
