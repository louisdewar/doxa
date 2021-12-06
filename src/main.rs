use std::{env, path::PathBuf, sync::Arc};

use actix_web::{web, App, HttpServer};

use doxa_auth::limiter::GenericLimiter;
use doxa_competition::{hello_world::HelloWorldCompetiton, CompetitionSystem};
use doxa_storage::AgentRetrieval;
use tracing::info;
use tracing_actix_web::TracingLogger;
use utt::UTTTCompetition;

mod telemetry;

fn create_competition_system(settings: doxa_competition::Settings) -> CompetitionSystem {
    // TODO: Maybe if competitions don't exist in the DB they should be auto created?
    let mut system = CompetitionSystem::new(Arc::new(settings));

    system.add_competition(HelloWorldCompetiton, 3);
    system.add_competition(UTTTCompetition, 10);

    system
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    telemetry::init_telemetry();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mq_url = env::var("MQ_URL").expect("MQ_URL must be set");
    let redis_url = env::var("REDIS_RATE_LIMITER_URL").expect("REDIS_RATE_LIMITER_URL must be set");

    let doxa_storage_path = env::var("DOXA_STORAGE").unwrap_or_else(|_| "dev/doxa_storage".into());
    let jwt_secret = env::var("DOXA_JWT_SECRET")
        .ok()
        .map(|s| s.into_bytes())
        .unwrap_or_else(doxa_auth::settings::generate_rand_jwt_secret);

    info!("JWT Secret length = {}", jwt_secret.len());

    let redis_pool = doxa_core::redis::establish_pool(redis_url, 500).await;

    let generic_limiter = Arc::new(GenericLimiter::new(redis_pool));

    let auth_settings = doxa_auth::Settings {
        jwt_secret: doxa_auth::settings::generate_jwt_hmac(&jwt_secret),
        allow_registration: false,
        generic_limiter: generic_limiter.clone(),
    };

    let storage_settings = doxa_storage::Settings {
        root: PathBuf::from(doxa_storage_path),
        generic_limiter: generic_limiter.clone(),
    };

    let executor_settings = doxa_executor::Settings {
        firecracker_path: PathBuf::from("./dev/firecracker"),
        kernel_img: PathBuf::from("./dev/vmlinux"),
        kernel_boot_args: "console=ttyS0 reboot=k panic=1 pci=off".to_string(),
        rootfs: PathBuf::from("./dev/rootfs.img"),
        agent_retrieval: AgentRetrieval::new(
            "http://localhost:3001/api/storage/download/".to_string(),
        ),
    };

    doxa_db::run_migrations(&doxa_db::establish_connection(&database_url));

    let db_pool = web::Data::new(doxa_db::establish_pool(&database_url));
    let mq_pool = web::Data::new(doxa_mq::establish_pool(mq_url, 25).await);

    doxa_mq::wait_for_mq(&mq_pool).await;

    let competition_settings = doxa_competition::Settings {
        executor_settings: Arc::new(executor_settings),
        mq_pool: Arc::clone(&mq_pool),
        pg_pool: Arc::clone(&db_pool),
    };
    let competition_system = create_competition_system(competition_settings);

    let configure_competition_routes = competition_system.start().await;

    info!("Starting at 127.0.0.1:3001");

    HttpServer::new(move || {
        let api_scope = web::scope("/api");
        App::new()
            .app_data(db_pool.clone())
            .app_data(mq_pool.clone())
            .service(
                api_scope.service(
                    // The configure happens before the scope is applied so the scope could be set to anything
                    // TODO: do some more testing with this, this feels a bit hacky, maybe make a
                    // function to configure these three routes
                    web::scope("")
                        .configure(doxa_auth::config(auth_settings.clone()))
                        .configure(doxa_storage::config(storage_settings.clone()))
                        .configure(configure_competition_routes.clone()),
                ),
            )
            .wrap(TracingLogger::default())
    })
    .bind(("0.0.0.0", 3001))?
    .run()
    .await
}
