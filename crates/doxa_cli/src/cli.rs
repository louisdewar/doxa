//! The definitions for CLI argument parsing with `clap`

use clap::{crate_version, AppSettings, Parser, Subcommand};

use crate::command::{agent::AgentCommands, auth::AuthCommands, raw::RawCommand};

#[derive(Parser)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(version(crate_version!()))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: MainCommands,
    #[clap(short, long, global = true)]
    /// Prints more information (useful for debugging)
    pub verbose: bool,
    #[clap(short = 'P', long = "user-profile", global = true)]
    /// Overwrite the default user profile to use by choosing a different username.
    pub profile: bool,
}

#[derive(Subcommand)]
pub enum MainCommands {
    #[clap(subcommand)]
    /// Commands for logging in, registering and other user actions
    User(AuthCommands),
    #[clap(subcommand)]
    /// Commands for using agents
    Agent(AgentCommands),
    // TODO: figure out how to use `hide` here to avoid showing this in help
    /// Make raw HTTP valuerequests to the server (advanced)
    Raw(RawCommand),
}
