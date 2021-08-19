use std::{env, path::PathBuf};

use actix_web::{web, App, HttpServer};

use doxa_competition::{
    hello_world::HelloWorldCompetiton,
    manager::{CompetitionManager, CompetitionManagerBuilder},
};
use tracing_actix_web::TracingLogger;
mod telemetry;

fn create_competition_manager() -> CompetitionManager {
    let mut builder = CompetitionManagerBuilder::new();

    builder.add_competition(HelloWorldCompetiton);

    builder.build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    telemetry::init_telemetry();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mq_url = env::var("MQ_URL").expect("MQ_URL must be set");

    let auth_settings = doxa_auth::Settings {
        // Obviously temporary, in future this should be a paramter that gets passed in maybe as a
        // config file, and the value itself should be a randomly generated string.
        jwt_secret: doxa_auth::settings::generate_jwt_hmac(b"jwt secret password"),
    };

    let storage_settings = doxa_storage::Settings {
        root: PathBuf::from("/tmp/doxa_storage"),
    };

    doxa_db::run_migrations(&doxa_db::establish_connection(&database_url));
    let mq_conn = doxa_mq::establish_mq_connection(&mq_url)
        .await
        .expect("couldn't connect to MQ");
    let competition_manager = create_competition_manager();

    let configure_competiton_routes = competition_manager.start(mq_conn);

    let db_pool = web::Data::new(doxa_db::establish_pool(&database_url));
    let mq_pool = web::Data::new(doxa_mq::establish_pool(mq_url, 25).await);

    println!("Starting at 127.0.0.1:3001");

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .app_data(mq_pool.clone())
            .configure(doxa_auth::config(auth_settings.clone()))
            .configure(doxa_storage::config(storage_settings.clone()))
            .configure(configure_competiton_routes.clone())
            .wrap(TracingLogger::default())
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
