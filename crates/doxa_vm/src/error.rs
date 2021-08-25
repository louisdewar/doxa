use std::io;

use derive_more::{Display, Error, From};
use doxa_firecracker_sdk::error::SpawnError;
use tokio::task::JoinError;

use crate::stream::ReadMessageError;

#[derive(From, Error, Display, Debug)]
pub enum ManagerError {
    #[from(forward)]
    Spawn(SpawnError),
    IO(io::Error),
    Join(JoinError),
}

#[derive(From, Error, Display, Debug)]
pub enum SendAgentError {
    IO(io::Error),
    ReadMessage(ReadMessageError),
    MissingReceivedMessage,
}
