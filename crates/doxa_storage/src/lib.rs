// Maybe rename doxa_agent

mod controller;
mod error;
mod limits;
mod retrieval;
mod route;
mod settings;
mod storage;

use actix_web::web::Data;
use limits::UploadLimits;
pub use settings::Settings;
use storage::LocalStorage;

pub use reqwest::Error as RetrievalError;
pub use retrieval::AgentRetrieval;

pub fn config(settings: Settings) -> impl FnOnce(&mut actix_web::web::ServiceConfig) {
    move |cfg| {
        let limiter = UploadLimits::new(settings.generic_limiter.clone());

        cfg.app_data(Data::new(limiter));
        cfg.app_data(Data::new(LocalStorage::new(settings.root)));
        route::config(cfg);
    }
}
