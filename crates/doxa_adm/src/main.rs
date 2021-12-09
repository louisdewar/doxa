use clap::Parser;
use cli::Cli;
use doxa_db::diesel::PgConnection;

// TODO: re-enable agent and competition
// mod agent;
mod cli;
// mod competition;
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
    }
    // let cli_config = clap::load_yaml!("cli.yml");
    // let authors = clap::crate_authors!("\n");
    // let app = clap::App::from(cli_config)
    //     .author(authors)
    //     .version(clap::crate_version!());

    // let matches = app.get_matches();

    // let subcommand = if let Some(subcommand) = matches.subcommand() {
    //     subcommand
    // } else {
    //     panic!("Missing subcommand");
    // };

    // let connection = get_db_connection(&matches);
    // doxa_db::run_migrations(&connection);

    // let sub_matches = subcommand.1;
    // match subcommand.0 {
    //     "user" => user_subcommand(sub_matches, &connection),
    //     "competition" => competition_subcommand(sub_matches, &connection),
    //     "agent" => agent_subcommand(sub_matches, &connection),
    //     "invite" => invite_subcommand(sub_matches, &connection),
    //     _ => panic!("unrecognised subcommand"),
    // }
}
