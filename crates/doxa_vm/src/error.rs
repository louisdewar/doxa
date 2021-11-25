use std::io;

use derive_more::{Display, Error, From};
use doxa_firecracker_sdk::error::SpawnError;
use tokio::task::JoinError;

use crate::stream::{ExpectMessageError, ReadMessageError, ReadPartError, SendStreamError};

pub use doxa_firecracker_sdk::error::ShutdownError;

#[derive(From, Error, Display, Debug)]
pub enum ManagerError {
    #[from(forward)]
    Spawn(SpawnError),
    TimeoutWaitingForVMConnection,
    IO(io::Error),
    Join(JoinError),
}

#[derive(From, Error, Display, Debug)]
pub enum SendAgentError<E> {
    IO(io::Error),
    ReadPart(ReadPartError),
    ReadMessage(ReadMessageError),
    DownloadAgentError(SendStreamError<E>),
    Expect(ExpectMessageError),
}

#[derive(From, Error, Display, Debug)]
pub enum ExecutionSpawnError {
    UnknownLanguage {
        language: String,
    },
    MissingRequiredOption {
        option: String,
    },
    #[from]
    IO(io::Error),
}

#[derive(Debug, Error, From, Display)]
pub(crate) enum ReceieveAgentError {
    IO(io::Error),
    InvalidFormatting,
    ExtractError,
    Timeout { during: String },
    ReadPartError(ReadPartError),
    ReadMessageError(ReadMessageError),
    Expect(ExpectMessageError),
}

#[derive(Debug, Error, From, Display)]
pub enum HandleMessageError {
    IO(io::Error),
    MissingSeparator,
    UnrecognisedPrefix,
    Reboot(ExecutionSpawnError),
}

#[derive(Debug, Error, From, Display)]
pub enum RebootAgentError {
    IO(io::Error),
    Read(ReadMessageError),
}
