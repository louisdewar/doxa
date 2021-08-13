use std::env;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let auth_settings = doxa_auth::Settings {
        jwt_secret: doxa_auth::settings::generate_jwt_hmac(b"jwt secret password"),
    };

    let db_pool = web::Data::new(doxa_db::establish_connection(&database_url));
    println!("Starting at 127.0.0.1:3001");
    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .configure(doxa_auth::config(auth_settings.clone()))
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
