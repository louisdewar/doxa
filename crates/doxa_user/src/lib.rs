use doxa_core::actix_web;

mod route;

pub fn config() -> impl FnOnce(&mut actix_web::web::ServiceConfig) {
    move |cfg| {
        route::config(cfg);
    }
}
