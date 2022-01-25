use std::sync::Arc;

use clap::StructOpt;
use climatehack::{dataset::Datasets, ClimateHackCompetition};
use doxa_server::{tracing::warn, CompetitionSystem};
use uttt::UTTTCompetition;

mod cli;

use cli::App;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(e) = dotenv::dotenv() {
        warn!(error=%e, error=?e, "failed to load .env")
    }

    let app: App = App::parse();

    let datasets = Arc::new(
        Datasets::load_from_directory(app.climatehack_datasets_dir)
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

    competition_system.add_competition(UTTTCompetition, 25);
    competition_system.add_competition(climatehack, 25);

    doxa_server::setup_server_from_env(true, competition_system).await
}
