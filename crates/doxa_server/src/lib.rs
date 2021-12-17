//! This crate enables conveniently setting up a new deployment of DOXA with specified
//! competitions.

use std::{env, path::PathBuf, sync::Arc};

use doxa_auth::limiter::GenericLimiter;
use doxa_core::actix_web::{web, App, HttpServer};
use doxa_storage::AgentRetrieval;
use tracing::info;
use tracing_actix_web::TracingLogger;

mod telemetry;

pub use doxa_competition::CompetitionSystem;

/// Uses well known environment variables for configuring the various parameters of the server
/// (e.g. database urls).
///
/// If `dotenv` is set to true then this will load the environment variables from a `.env` in the
/// current directory or parents (if it exists).
///
/// Once the parameters are loaded this calls [`setup_server`].
pub async fn setup_server_from_env(
    use_dotenv: bool,
    competition_system: CompetitionSystem,
) -> std::io::Result<()> {
    if use_dotenv {
        // TODO: do not panic when dotenv doesn't exist
        dotenv::dotenv().expect("failed to load .env vars");
    }

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
        firecracker_path: PathBuf::from("./dev/vm/firecracker"),
        kernel_img: PathBuf::from("./dev/vm/vmlinux"),
        kernel_boot_args: "console=ttyS0 reboot=k panic=1 pci=off".to_string(),
        rootfs: PathBuf::from("./dev/vm/rootfs.img"),
        agent_retrieval: AgentRetrieval::new(
            "http://localhost:3001/api/storage/download/".to_string(),
        ),
    };

    setup_server(
        &database_url,
        &mq_url,
        auth_settings,
        storage_settings,
        executor_settings,
        competition_system,
    )
    .await
}

/// Sets up server based on the given settings for each system.
/// This will start the server and run until exit.
///
/// This will also automatically initialize telemetry and run database migrations.
pub async fn setup_server(
    database_url: &str,
    mq_url: &str,
    auth_settings: doxa_auth::Settings,
    storage_settings: doxa_storage::Settings,
    executor_settings: doxa_executor::Settings,
    competition_system: CompetitionSystem,
) -> std::io::Result<()> {
    telemetry::init_telemetry();

    doxa_db::run_migrations(&doxa_db::establish_connection(database_url));

    let db_pool = web::Data::new(doxa_db::establish_pool(database_url));
    let mq_pool = web::Data::new(doxa_mq::establish_pool(mq_url.to_string(), 25).await);

    doxa_mq::wait_for_mq(&mq_pool).await;

    let competition_settings = doxa_competition::Settings {
        executor_settings: Arc::new(executor_settings),
        mq_pool: Arc::clone(&mq_pool),
        pg_pool: Arc::clone(&db_pool),
    };

    let configure_competition_routes = competition_system
        .start(Arc::new(competition_settings))
        .await;

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
                        .configure(doxa_user::config())
                        .configure(configure_competition_routes.clone()),
                ),
            )
            .wrap(TracingLogger::default())
    })
    .bind(("0.0.0.0", 3001))?
    .run()
    .await
}
