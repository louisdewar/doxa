use clap::StructOpt;
use climatehack::{ClimateHackCompetition, PhaseDataset};
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

    let dataset = PhaseDataset::new(app.dataset_y_folder, app.dataset_x_image).await;
    let climatehack = ClimateHackCompetition {
        dataset,
        python_bin: app.scorer_python_bin,
    };

    let mut competition_system = CompetitionSystem::new();

    competition_system.add_competition(UTTTCompetition, 25);
    competition_system.add_competition(climatehack, 25);

    doxa_server::setup_server_from_env(true, competition_system).await
}
