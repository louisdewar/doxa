//! The definitions for CLI argument parsing with `clap`

use clap::{Parser, Subcommand};

use crate::command::{agent::AgentCommands, auth::AuthCommands};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: MainCommands,
    #[clap(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum MainCommands {
    #[clap(subcommand)]
    User(AuthCommands),
    #[clap(subcommand)]
    Agent(AgentCommands),
}
