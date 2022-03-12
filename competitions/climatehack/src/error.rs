use std::io;

use derive_more::{Display, Error, From};
use doxa_competition::client::{serde_json, ForfeitError, GameError};

#[derive(Error, Debug, Display, From)]
pub enum ClimateHackError {
    Scorer(ScorerError),
    #[display(
        fmt = "agent outputted invalid message (message=`{}`), expected `{}`",
        message,
        expected
    )]
    InvalidMessage {
        message: String,
        expected: String,
    },
    InvalidStartupMessage {
        message: String,
    },
    TimeoutStartup,
    TimeoutGroup,
    #[from(ignore)]
    WriteGroupError(io::Error),
    #[from(forward)]
    UnknownDataset(UnknownDataset),
}

impl From<ClimateHackError> for GameError<ClimateHackError> {
    fn from(e: ClimateHackError) -> GameError<ClimateHackError> {
        GameError::Client(e)
    }
}

impl ForfeitError for ClimateHackError {
    fn forfeit(&self) -> Option<usize> {
        match self {
            ClimateHackError::Scorer(e) => e.forfeit(),
            ClimateHackError::InvalidMessage { .. } => Some(0),
            ClimateHackError::InvalidStartupMessage { .. } => Some(0),
            ClimateHackError::TimeoutStartup => Some(0),
            ClimateHackError::TimeoutGroup => Some(0),
            ClimateHackError::WriteGroupError(_) => None,
            ClimateHackError::UnknownDataset(_) => None,
        }
    }

    fn forfeit_message(&self) -> Option<String> {
        match self {
            ClimateHackError::Scorer(e) => e.forfeit_message(),
            ClimateHackError::InvalidMessage { message, expected } => Some(format!(
                "The agent outputted an invalid message (message=`{}`), expected `{}`",
                message, expected
            )),
            ClimateHackError::InvalidStartupMessage { message } => Some(format!(
                "The agent did not output the correct startup message, they instead output `{}`",
                message
            )),
            ClimateHackError::TimeoutStartup => {
                Some("The agent too long to startup and produce the startup message".into())
            }
            ClimateHackError::TimeoutGroup => {
                Some("The agent too long to process an input group".into())
            }
            ClimateHackError::WriteGroupError(_) => None,
            ClimateHackError::UnknownDataset(_) => None,
        }
    }
}

#[derive(Error, Debug, Display)]
pub enum DatasetLoadingError {
    ReadDir(io::Error),
    DatasetX(io::Error),
    DatasetY(io::Error),
    DatasetXNotFile,
    DatasetXNotDirectory,
    DatasetYNotDirectory,
}

#[derive(Error, Debug, Display)]
#[display(fmt = "Cannot find the dataset with the name `{}`", name)]
pub struct UnknownDataset {
    pub name: String,
}

#[derive(Error, Debug, Display)]
pub enum ScorerError {
    #[display(fmt = "internal error scoring agent: {}", _0)]
    InternalScriptError(#[error(not(source))] String),
    #[display(fmt = "agent forfeited: {}", error)]
    ForfeitError {
        error: String,
        forfeit: String,
    },
    WriteScript(io::Error),
    Format(serde_json::Error),
    StartScript(io::Error),
    ScriptOutput(io::Error),
}

impl ForfeitError for ScorerError {
    fn forfeit(&self) -> Option<usize> {
        match self {
            ScorerError::WriteScript(_) => None,
            ScorerError::Format(_) => None,
            ScorerError::StartScript(_) => None,
            ScorerError::ScriptOutput(_) => None,
            ScorerError::InternalScriptError(_) => None,
            ScorerError::ForfeitError { .. } => Some(0),
        }
    }

    fn forfeit_message(&self) -> Option<String> {
        match self {
            ScorerError::WriteScript(_) => None,
            ScorerError::Format(_) => None,
            ScorerError::StartScript(_) => None,
            ScorerError::ScriptOutput(_) => None,
            ScorerError::InternalScriptError(_) => None,
            ScorerError::ForfeitError { forfeit, .. } => Some(forfeit.to_string()),
        }
    }
}
