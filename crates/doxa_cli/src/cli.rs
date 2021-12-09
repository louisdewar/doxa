//! The definitions for CLI argument parsing with `clap`

use clap::{Parser, Subcommand};

use crate::command::{agent::AgentCommands, auth::AuthCommands};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: MainCommands,
}

#[derive(Subcommand)]
pub enum MainCommands {
    #[clap(flatten)]
    Auth(AuthCommands),
    #[clap(subcommand)]
    Agent(AgentCommands),
}
