use std::{io, process::ExitStatus};

// TODO: clearly these errors need to be partitioned into host and guest.
// There are many situations where they is a different but equivalent error for both sides,
// this may get messy.

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
    #[from(ignore)]
    CreateScratch(RunCommandError),
    #[from(ignore)]
    CreateSwap(RunCommandError),
    #[from(ignore)]
    GetImageUUID(RunCommandError),
    #[from]
    Mount(MountError),
    Bollard(bollard::errors::Error),
    DockerMissingResponse,
    MissingBridgeNetwork,
}

#[derive(Error, Display, Debug)]
#[display(fmt = "{}", source)]
pub struct ManagerErrorLogContext {
    pub source: ManagerError,
    pub logs: Option<Result<String, VMRecorderError>>,
}

impl<E: Into<ManagerError>> From<E> for ManagerErrorLogContext {
    fn from(source: E) -> Self {
        ManagerErrorLogContext {
            source: source.into(),
            logs: None,
        }
    }
}

#[derive(From, Error, Display, Debug)]
pub enum MountError {
    IO(io::Error),
    Expect(ExpectMessageError),
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
pub enum TakeFileError {
    FileNotFound,
    FileTooLarge,
    #[display(fmt = "the provided path was not a file")]
    NotFile,
    #[from]
    Other(io::Error),
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

#[derive(Error, Display, Debug)]
pub enum AgentShutdownError {
    FailedToKillAgent(io::Error),
    AgentNotRunning,
}

#[derive(From, Error, Display, Debug)]
pub enum AgentLifecycleError {
    #[from]
    Spawn(ExecutionSpawnError),
    #[from]
    Shutdown(AgentShutdownError),
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
pub(crate) enum HandleMountsError {
    IO(io::Error),
    InvalidFormatting,
    InvalidSwapMessage,
    ReadMessageError(ReadMessageError),
    #[from(ignore)]
    FindDrives(RunCommandError),
    #[display(
        fmt = "UUID not found (UUID=\"{}\", mount path=\"{}\")",
        uuid,
        mount_path
    )]
    UUIDNotFound {
        uuid: String,
        mount_path: String,
    },
    #[from(ignore)]
    ActivateSwap(RunCommandError),
}

#[derive(Debug, Error, From, Display)]
pub enum HandleMessageError {
    IO(io::Error),
    MissingSeparator,
    UnrecognisedPrefix,
    Lifecycle(AgentLifecycleError),
    TakeFile(TakeFileError),
}

#[derive(Debug, Error, From, Display)]
pub enum AgentLifecycleManagerError {
    IO(io::Error),
    Read(ReadMessageError),
    Timeout,
    MissingSpawnedMessage,
}

#[derive(From, Error, Display, Debug)]
pub enum TakeFileManagerError {
    #[from]
    ReadMessage(ReadMessageError),
    Timeout,
    #[from]
    IO(io::Error),
}

#[derive(From, Error, Display, Debug)]
pub enum VMRecorderError {
    Timeout,
    #[from]
    IO(io::Error),
    #[from]
    Join(JoinError),
}

#[derive(From, Error, Display, Debug)]
pub enum VMShutdownError {
    #[from(forward)]
    Firecracker(ShutdownError),
    Bollard(bollard::errors::Error),
    #[from]
    Logs(VMRecorderError),
}

#[derive(From, Error, Display, Debug)]
pub enum RunCommandError {
    #[from]
    IO(io::Error),
    #[from]
    #[error(ignore)]
    BadExitCode(ExitStatus),
}
