use clap::Parser;
use cli::Cli;
use doxa_db::diesel::PgConnection;

// TODO: re-enable agent and competition
// mod agent;
mod cli;
mod competition;
mod invite;
mod user;

fn get_db_connection() -> PgConnection {
    dotenv::dotenv().expect("Couldn't read .env");
    let url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set or provided as an argument");

    doxa_db::establish_connection(&url)
}

fn main() {
    let args = Cli::parse();

    let connection = get_db_connection();
    doxa_db::run_migrations(&connection);

    match args.command {
        cli::MainCommands::User(subcommand) => user::handle_subcommand(subcommand, &connection),
        cli::MainCommands::Invite(subcommand) => invite::handle_subcommand(subcommand, &connection),
        cli::MainCommands::Competition(subcommand) => {
            competition::handle_subcommand(subcommand, &connection)
        }
    }
}
