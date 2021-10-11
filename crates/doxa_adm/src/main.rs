use clap::ArgMatches;
use doxa_db::diesel::PgConnection;

use crate::{
    agent::agent_subcommand, competition::competition_subcommand, invite::invite_subcommand,
};

mod agent;
mod competition;
mod invite;
mod user;

fn get_db_connection(matches: &ArgMatches) -> PgConnection {
    let url = if let Some(url) = matches.value_of("database_url") {
        url.to_owned()
    } else {
        dotenv::dotenv().expect("Couldn't read .env");
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set or provided as an argument")
    };

    doxa_db::establish_connection(&url)
}

fn main() {
    let cli_config = clap::load_yaml!("cli.yml");
    let authors = clap::crate_authors!("\n");
    let app = clap::App::from(cli_config)
        .author(authors)
        .version(clap::crate_version!());

    let matches = app.get_matches();

    let subcommand = if let Some(subcommand) = matches.subcommand() {
        subcommand
    } else {
        panic!("Missing subcommand");
    };

    let connection = get_db_connection(&matches);
    doxa_db::run_migrations(&connection);

    let sub_matches = subcommand.1;
    match subcommand.0 {
        "user" => user_subcommand(sub_matches, &connection),
        "competition" => competition_subcommand(sub_matches, &connection),
        "agent" => agent_subcommand(sub_matches, &connection),
        "invite" => invite_subcommand(sub_matches, &connection),
        _ => panic!("unrecognised subcommand"),
    }
}

fn user_subcommand(matches: &ArgMatches, conn: &PgConnection) {
    let subcommand = matches.subcommand().expect("missing user subcommand");
    let sub_matches = subcommand.1;

    match subcommand.0 {
        "list" => user::list_users(sub_matches, conn),
        "admin" => user_admin_subcommand(sub_matches, conn),
        _ => panic!("unrecognised user subcommand"),
    }
}

fn user_admin_subcommand(matches: &ArgMatches, conn: &PgConnection) {
    let subcommand = matches
        .subcommand()
        .expect("missing `user admin` subcommand");
    let sub_matches = subcommand.1;

    match subcommand.0 {
        "list" => user::list_admins(sub_matches, conn),
        "promote" => user::set_admin_status(sub_matches, conn, true),
        "demote" => user::set_admin_status(sub_matches, conn, false),
        _ => panic!("unrecognised user subcommand"),
    }
}
