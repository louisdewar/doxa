use std::{env, path::PathBuf};

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let auth_settings = doxa_auth::Settings {
        jwt_secret: doxa_auth::settings::generate_jwt_hmac(b"jwt secret password"),
    };

    let storage_settings = doxa_storage::Settings {
        root: PathBuf::from("/tmp/doxa_storage"),
    };

    doxa_db::run_migrations(&doxa_db::establish_connection(&database_url));

    let db_pool = web::Data::new(doxa_db::establish_pool(&database_url));
    println!("Starting at 127.0.0.1:3001");
    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .configure(doxa_auth::config(auth_settings.clone()))
            .configure(doxa_storage::config(storage_settings.clone()))
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
