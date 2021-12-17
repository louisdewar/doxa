use doxa_server::CompetitionSystem;
use uttt::UTTTCompetition;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut competition_system = CompetitionSystem::new();

    competition_system.add_competition(UTTTCompetition, 10);

    doxa_server::setup_server_from_env(true, competition_system).await
}
