//! This crate enables conveniently setting up a new deployment of DOXA with specified
//! competitions.

use std::{env, path::PathBuf, sync::Arc};

use doxa_auth::{limiter::GenericLimiter, AuthaClient};
use doxa_core::actix_web::{web, App, HttpServer};
use doxa_executor::settings::Mount;
use doxa_storage::AgentRetrieval;
use tracing::{info, warn};
use tracing_actix_web::TracingLogger;

mod telemetry;

pub use doxa_competition::CompetitionSystem;
pub use doxa_core::tracing;

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
    telemetry::init_telemetry();

    if use_dotenv {
        if let Err(e) = dotenv::dotenv() {
            warn!(error=%e, debug=?e, "failed to read .env");
        }
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mq_url = env::var("MQ_URL").expect("MQ_URL must be set");
    let redis_url = env::var("REDIS_DB_URL").expect("REDIS_DB_URL must be set");

    let doxa_storage_path = env::var("DOXA_STORAGE").unwrap_or_else(|_| "dev/doxa_storage".into());

    let redis_pool = doxa_core::redis::establish_pool(redis_url, 500).await;

    let generic_limiter = Arc::new(GenericLimiter::new(redis_pool.clone()));

    let autha_base_url = env::var("AUTHA_BASE_URL").unwrap();
    let autha_shared_secret = env::var("AUTHA_SHARED_SECRET").unwrap();
    let delegated_auth_redirect = env::var("DOXA_DELEGATED_AUTH_URL").unwrap();
    let system_account_secret = env::var("DOXA_SYSTEM_ACCOUNT_SECRET").unwrap();

    let autha_client = Arc::new(
        AuthaClient::new(
            autha_base_url
                .parse()
                .expect("AUTHA_BASE_URL was not parsable base url"),
            autha_shared_secret,
        )
        .await
        .expect("failed to startup autha client"),
    );

    let auth_settings = doxa_auth::Settings {
        allow_registration: false,
        autha_client,
        redis_db: redis_pool.clone(),
        delegated_auth_url: delegated_auth_redirect
            .parse()
            .expect("The delegated auth URL is not valid"),
        system_account_secret,
    };

    let storage_settings = doxa_storage::Settings {
        root: PathBuf::from(doxa_storage_path),
        generic_limiter: generic_limiter.clone(),
    };

    let executor_settings = doxa_executor::Settings {
        firecracker_path: PathBuf::from("./dev/vm/firecracker"),
        kernel_img: PathBuf::from("./dev/vm/vmlinux"),
        kernel_boot_args: "console=ttyS0 reboot=k panic=1 pci=off".to_string(),
        rootfs: PathBuf::from("./dev/vm/images/rootfs.img"),
        scratch_base_image: PathBuf::from("./dev/vm/images/scratch.img"),
        agent_retrieval: AgentRetrieval::new(
            "http://localhost:3001/api/storage/download/".to_string(),
            auth_settings.system_account_secret.clone(),
        ),
        base_mounts: vec![Mount {
            path_on_host: PathBuf::from("./dev/vm/images/python_modules.img"),
            path_on_guest: "/python_env".to_string(),
            read_only: true,
        }],
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
    doxa_db::run_migrations(&doxa_db::establish_connection(database_url));

    let db_pool = web::Data::new(doxa_db::establish_pool(database_url));
    let mq_pool = web::Data::new(doxa_mq::establish_pool(mq_url.to_string(), 25).await);

    doxa_mq::wait_for_mq(&mq_pool).await;

    let competition_settings = doxa_competition::Settings {
        executor_settings: Arc::new(executor_settings),
        mq_pool: Arc::clone(&mq_pool),
        pg_pool: Arc::clone(&db_pool),
        generic_limiter: storage_settings.generic_limiter.clone(),
        request_client: doxa_competition::settings::HTTPClient::new(),
        competitions_base_url: "http://localhost:3001/api/competition/".to_string(),
    };

    let configure_competition_routes = competition_system
        .start(Arc::new(competition_settings))
        .await;

    info!("Starting at 0.0.0.0:3001");

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
