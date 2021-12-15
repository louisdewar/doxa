use clap::Parser;
use cli::Cli;
use error::{CliError, CommandError, RequestError};

use crate::request::{parse_base_url, Settings};

pub mod config;
pub mod error;
pub mod request;
pub mod ui;

mod cli;
mod command;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        if let CliError::Command(CommandError::Request(RequestError::Doxa(doxa))) = &e {
            if let Some(message) = &doxa.message {
                ui::error(message);
                return;
            }
        }

        ui::error(e);
    }
}

async fn run() -> Result<(), CliError> {
    let args = Cli::parse();

    let config_dir = config::default_config_dir();

    let profiles = config::load_or_default_profile(&config_dir).await?;

    let base_url = std::env::var("DOXA_BASE_URL")
        .unwrap_or_else(|_| "https://doxa.uclaisociety.co.uk/".to_string());

    // let user_profile = matches
    //     .value_of("USER_PROFILE")
    //     .map(|username| profiles.user_profile(username))
    //     .unwrap_or_else(|| profiles.default_profile());

    let user_profile = profiles.default_profile()?;

    let base_url = parse_base_url(&base_url)?;

    let verbose = args.verbose;
    let settings = Settings::new(user_profile, base_url, config_dir, verbose);

    match args.command {
        cli::MainCommands::User(auth) => command::auth::handle_subcommand(auth, &settings).await?,
        cli::MainCommands::Agent(agent) => {
            command::agent::handle_subcommand(agent, &settings).await?
        }
    }

    Ok(())
}
