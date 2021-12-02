use error::CliError;

use crate::request::{parse_base_url, Settings};

pub mod command;
pub mod config;
pub mod error;
pub mod request;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("failed: {}", e);
    }
}

async fn run() -> Result<(), CliError> {
    let cli_config = clap::load_yaml!("cli.yml");
    let authors = clap::crate_authors!("\n");
    let app = clap::App::from(cli_config)
        .author(authors)
        .version(clap::crate_version!());
    let matches = app.get_matches();

    let config_dir = config::default_config_dir();

    let profiles = config::load_or_default_profile(&config_dir).await.unwrap();

    let base_url = std::env::var("DOXA_BASE_URL")
        .unwrap_or_else(|_| "https://doxa.uclaisociety.co.uk/".to_string());

    let base_url = parse_base_url(&base_url)?;

    let user_profile = matches
        .value_of("USER_PROFILE")
        .map(|username| profiles.user_profile(username))
        .unwrap_or_else(|| profiles.default_profile());

    let settings = Settings::new(user_profile.cloned(), base_url, config_dir);

    let subcommand = if let Some(subcommand) = matches.subcommand() {
        subcommand
    } else {
        panic!("Missing subcommand");
    };

    let sub_matches = subcommand.1;
    match subcommand.0 {
        "login" => command::auth::login(sub_matches, &settings).await,
        "register" => command::auth::register(sub_matches, &settings).await,
        "agent" => command::agent::subcommand(sub_matches, &settings).await,
        _ => panic!("unrecognised subcommand"),
    }?;

    Ok(())
}
