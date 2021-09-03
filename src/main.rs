use std::{env, path::PathBuf, sync::Arc};

use actix_web::{web, App, HttpServer};

use doxa_competition::{hello_world::HelloWorldCompetiton, CompetitionSystem};
use doxa_storage::AgentRetrieval;
use tracing_actix_web::TracingLogger;
mod telemetry;

fn create_competition_system(settings: doxa_competition::Settings) -> CompetitionSystem {
    let mut system = CompetitionSystem::new(Arc::new(settings));

    system.add_competition(HelloWorldCompetiton);

    system
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
        root: PathBuf::from("dev/doxa_storage"),
    };

    let executor_settings = doxa_executor::Settings {
        firecracker_path: PathBuf::from("./dev/firecracker"),
        kernel_img: PathBuf::from("./dev/vmlinux.bin"),
        kernel_boot_args: "console=ttyS0 reboot=k panic=1 pci=off".to_string(),
        rootfs: PathBuf::from("./dev/rootfs.img"),
        agent_retrieval: AgentRetrieval::new("http://localhost:3001/storage/download/".to_string()),
    };

    doxa_db::run_migrations(&doxa_db::establish_connection(&database_url));

    let db_pool = web::Data::new(doxa_db::establish_pool(&database_url));
    let mq_pool = web::Data::new(doxa_mq::establish_pool(mq_url, 25).await);

    let competition_settings = doxa_competition::Settings {
        executor_settings: Arc::new(executor_settings),
        mq_pool: Arc::clone(&mq_pool),
        pg_pool: Arc::clone(&db_pool),
    };
    let competition_system = create_competition_system(competition_settings);
    let configure_competiton_routes = competition_system.generate_configure_fn();

    competition_system.start().await;

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
