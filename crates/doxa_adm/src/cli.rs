//! The definitions for CLI argument parsing with `clap`

use clap::{Parser, Subcommand};

use crate::{competition::CompetitionCommands, invite::InviteCommands, user::UserCommands};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: MainCommands,
}

#[derive(Subcommand)]
pub enum MainCommands {
    #[clap(subcommand)]
    /// Commands for managing users
    User(UserCommands),
    #[clap(subcommand)]
    /// Commands for managing invites
    Invite(InviteCommands),
    #[clap(subcommand)]
    /// Commands for managing competitions
    Competition(CompetitionCommands),
}
