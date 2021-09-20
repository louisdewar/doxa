use std::{convert::Infallible, io};

use derive_more::{Display, Error, From};
use doxa_core::lapin;
use doxa_mq::action::BincodeError;
use doxa_storage::RetrievalError;
use doxa_vm::{
    error::{ManagerError, SendAgentError},
    stream::ReadMessageError,
};

/// A way of indicating whether an error should count as a forfeit for a particular agent
pub trait ForfeitError {
    fn forfeit(&self) -> Option<usize>;
}

#[derive(From, Error, Display, Debug)]
pub struct Timeout {
    pub during: String,
}

#[derive(From, Error, Display, Debug)]
pub enum AgentError {
    IO(io::Error),
    /// When downloading the agent 404 was returned this could easily happen in situations where a new
    /// agent was uploaded since the game was queued.
    AgentNotFound,
    /// A retrieval error that occurs before the download begins
    Request(RetrievalError),
    /// The status code when downloading was not 200 (and not 404 because that is handled elsewhere)
    BadStatusCode,
    /// An error while sending the agent to the VM, this happens synchronously while downloading
    /// the agent so it could be an internal server error that occured as part of the download.
    SendAgentError(SendAgentError<RetrievalError>),
    /// An internal VM error
    VM(ManagerError),
    /// There was an error extracting the filename of the agent from the server
    CouldNotExtractFilename,
    /// There was an error extracting the file size of the agent from the server
    CouldNotExtractFileSize,
    /// An operation timedout
    Timeout(Timeout),
    /// Failed to read a message across the socket
    Socket(ReadMessageError),
}

#[derive(Display, Error, From, Debug)]
pub enum GameContextError {
    /// The provided id did not exist.
    #[display(fmt = "unknown agent with id `{}` the max was `{}`", id, max)]
    #[from(ignore)]
    UnknownAgent {
        id: usize,
        /// The maximum is the largest allowed agent id
        max: usize,
    },
    NextMessage(NextMessageError),
    #[from(ignore)]
    SendInput(io::Error),
    #[display(fmt = "failed to deserialize payload: {}", _0)]
    PayloadDeserialize(BincodeError),
    #[display(fmt = "failed to emit event: {}", _0)]
    Emit(lapin::Error),
    #[display(
        fmt = "ran out of time while waiting for next message from agent (assigned id={})",
        agent_id
    )]
    #[from(ignore)]
    TimeoutWaitingForMessage {
        agent_id: usize,
    },
    #[display(
        fmt = "the game expected {} agents when in reality there were {}",
        expected,
        actual
    )]
    #[from(ignore)]
    IncorrectNumberAgents {
        expected: usize,
        actual: usize,
    },
    #[display(fmt = "the client tried to emit a zero length event type")]
    ZeroLengthEventType,
    #[display(
        fmt = "the client tried to emit an event type that began with an underscore which is reserved for system events"
    )]
    ReservedEventType,
}

impl ForfeitError for GameContextError {
    fn forfeit(&self) -> Option<usize> {
        match &self {
            GameContextError::UnknownAgent { .. } => None,
            // TODO: next message / send input should both be forfeit errors but we need the agent
            // id
            GameContextError::NextMessage(_) => None,
            GameContextError::SendInput(_) => None,
            GameContextError::PayloadDeserialize(_) => None,
            GameContextError::Emit(_) => None,
            GameContextError::TimeoutWaitingForMessage { agent_id } => Some(*agent_id),
            GameContextError::IncorrectNumberAgents { .. } => None,
            GameContextError::ZeroLengthEventType => None,
            GameContextError::ReservedEventType => None,
        }
    }
}

#[derive(Display, Error, From, Debug)]
pub enum GameError<E> {
    Context(GameContextError),
    #[from(ignore)]
    Client(E),
}

impl<E: ForfeitError> ForfeitError for GameError<E> {
    fn forfeit(&self) -> Option<usize> {
        match &self {
            GameError::Context(e) => e.forfeit(),
            GameError::Client(e) => e.forfeit(),
        }
    }
}

impl ForfeitError for Infallible {
    fn forfeit(&self) -> Option<usize> {
        None
    }
}

#[derive(Display, Error, From, Debug)]
pub struct AgentShutdown;

#[derive(Display, Error, From, Debug)]
pub enum NextEventError {
    /// An error occured reading the message
    ReadMessage(ReadMessageError),
    UnrecognisedPrefix,
    MissingSeparator,
}

#[derive(Display, Error, From, Debug)]
pub enum NextMessageError {
    NextEvent(NextEventError),
    /// The agent process has terminated successfully either previously or while waiting for the current message
    Shutdown(AgentShutdown),
}

// TODO: either remove entierly or just remove runtime OR change on_game_error to take in
// GameManagerError and then include startup errors
#[derive(Display, Error, From, Debug)]
pub enum GameManagerError<E> {
    #[from(ignore)]
    StartAgent(AgentError),
    Runtime(GameError<E>),
}
