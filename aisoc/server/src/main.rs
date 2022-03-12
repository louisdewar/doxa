use std::{env, path::PathBuf, sync::Arc};

use clap::StructOpt;
use climatehack::{dataset::Datasets, ClimateHackCompetition};
use doxa_execution_node::manager::docker::{self, Docker, DockerCredentials};
use doxa_executor::settings::AgentRetrieval;
use doxa_server::{tracing::warn, CompetitionSystem};
use url::Url;
use uttt::UTTTCompetition;

mod cli;

use cli::App;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(e) = dotenv::dotenv() {
        warn!(error=%e, error=?e, "failed to load .env")
    }

    let app: App = App::parse();

    match app.subcommands {
        cli::Subcommands::Main => {
            let datasets = Arc::new(
                Datasets::load_from_directory(app.climatehack_datasets_dir, true)
                    .await
                    .unwrap(),
            );
            // let dataset = PhaseDataset::new(app.dataset_y_folder, app.dataset_x_image).await;
            let climatehack = ClimateHackCompetition {
                datasets,
                primary_dataset: app.climatehack_primary_dataset,
                python_bin: app.scorer_python_bin,
            };
            let mut competition_system = CompetitionSystem::new();

            competition_system.add_competition(UTTTCompetition, 15);
            competition_system.add_competition(climatehack, 3);

            doxa_server::setup_server_from_env(true, competition_system).await
        }
        cli::Subcommands::ExecutionNode {
            api_base_url,
            runtime,
            docker_username,
            docker_password,
        } => {
            doxa_server::telemetry::init_telemetry();
            use doxa_execution_node::manager::{
                CompetitionManagerSettings, CompetitionNodeManager,
            };

            let datasets = Arc::new(
                Datasets::load_from_directory(app.climatehack_datasets_dir, false)
                    .await
                    .unwrap(),
            );
            // let dataset = PhaseDataset::new(app.dataset_y_folder, app.dataset_x_image).await;
            let climatehack = ClimateHackCompetition {
                datasets,
                primary_dataset: app.climatehack_primary_dataset,
                python_bin: app.scorer_python_bin,
            };
            let mq_url = env::var("MQ_URL").expect("MQ_URL must be set");
            let system_account_secret = env::var("DOXA_SYSTEM_ACCOUNT_SECRET")
                .expect("DOXA_SYSTEM_ACCOUNT_SECRET must be set");
            let mq_pool = doxa_mq::establish_pool(mq_url.to_string(), 25).await;

            let docker_settings = docker::DockerBackendSettings {
                image: "registry.dewardt.uk/doxa/evaluation_environment".to_string(),
                //image: "registry.dewardt.uk/doxa/evaluation_environment".to_string(),
                docker: Docker::connect_with_unix_defaults().unwrap(),
                credentials: Some(DockerCredentials {
                    username: docker_username,
                    password: docker_password,
                    ..Default::default()
                }),
                runtime,
                memory_limit_bytes: None,
            };
            let settings = CompetitionManagerSettings {
                executor_permits: 1,
                api_base_url: api_base_url.clone(),
                request_client: Default::default(),
                mq_pool,
                executor_settings: Arc::new(doxa_executor::Settings {
                    agent_retrieval: AgentRetrieval::new(
                        "http://doxa.uclaisociety.co.uk/api/storage/download/".to_string(),
                        system_account_secret,
                    ),
                    scratch_base_image: PathBuf::from("./dev/vm/images/scratch.img"),
                    base_mounts: Vec::new(),
                }),
                docker_settings,
            };

            let competition_node_manager =
                CompetitionNodeManager::new(Arc::new(climatehack), settings);
            competition_node_manager.start().await;

            Ok(())
        }
    }
}
