use std::io;

use derive_more::{Display, Error, From};
use doxa_core::lapin;
use doxa_mq::action::BincodeError;
use doxa_storage::RetrievalError;
use doxa_vm::{
    error::{ManagerError, SendAgentError},
    stream::ReadMessageError,
};

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
}

#[derive(Display, Error, From, Debug)]
pub enum GameError<E> {
    Context(GameContextError),
    #[from(ignore)]
    Client(E),
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

#[derive(Display, Error, From, Debug)]
pub enum GameManagerError<E> {
    #[from(ignore)]
    StartAgent(AgentError),
    Runtime(GameError<E>),
}
