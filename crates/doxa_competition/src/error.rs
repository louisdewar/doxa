use derive_more::{Display, Error, From};
use doxa_core::{
    actix_web, deadpool_lapin, impl_respondable_error, lapin, tokio::task::JoinError,
    RespondableError,
};
use doxa_db::{diesel::r2d2, DieselError};

#[derive(From, Error, Display, Debug, RespondableError)]
/// A context error for a particular competition (not to be confused with the context error from an
/// execution
pub enum ContextError {
    #[from]
    MessageQueue(lapin::Error),
    #[from]
    MessageQueuePool(deadpool_lapin::PoolError),
    #[from]
    DatabaseConnection(r2d2::PoolError),
    #[from]
    Join(JoinError),
    #[from]
    Diesel(DieselError),
    #[from]
    AgentNotFound(AgentNotFound),
    #[from]
    ParseSystemMessage(ParseSystemMessageError),
    #[from]
    StartEventNotFound(StartEventNotFound),
}

#[derive(From, Error, Display, Debug)]
/// Errors that occur from the competition managers
pub enum CompetitionManagerError {
    #[from]
    DatabaseConnection(r2d2::PoolError),
    #[from]
    Join(JoinError),
    #[from]
    Diesel(DieselError),
}

#[derive(Error, Display, Debug)]
#[display(fmt = "no game could be found with id {} in this competition", game_id)]
pub struct GameNotFound {
    pub game_id: i32,
}

impl_respondable_error!(
    GameNotFound,
    NOT_FOUND,
    "GAME_NOT_FOUND",
    "No game could be found with that id in this competition"
);

#[derive(Error, Display, Debug)]
#[display(
    fmt = "failed to parse system message `{}` for game ID `{}`: {}",
    event_type,
    game_id,
    error
)]
pub struct ParseSystemMessageError {
    pub event_type: String,
    pub game_id: i32,
    pub error: serde_json::Error,
}

impl_respondable_error!(
    ParseSystemMessageError,
    INTERNAL_SERVER_ERROR,
    "INTERNAL_SERVER_ERROR"
);

#[derive(Error, Display, Debug)]
#[display(fmt = "could not find the `_START` event for game `{}`", game_id)]
pub struct StartEventNotFound {
    pub game_id: i32,
}

impl_respondable_error!(
    StartEventNotFound,
    INTERNAL_SERVER_ERROR,
    "INTERNAL_SERVER_ERROR"
);

#[derive(Error, Display, Debug)]
pub struct NoActiveAgent;

impl_respondable_error!(
    NoActiveAgent,
    BAD_REQUEST,
    "NO_ACTIVE_AGENT",
    "There is no active agent for this user"
);

#[derive(Error, Display, Debug)]
pub struct AgentNotFound;

impl_respondable_error!(
    AgentNotFound,
    NOT_FOUND,
    "AGENT_NOT_FOUND",
    "No agent with that ID is part of this competition"
);

#[derive(Error, Display, Debug)]
#[display(
    fmt = "there was at least one event but it didn't have type _START the type was `{}`",
    event_type
)]
pub struct MissingStartEvent {
    pub event_type: String,
}

impl_respondable_error!(
    MissingStartEvent,
    INTERNAL_SERVER_ERROR,
    "INTERNAL_SERVER_ERROR"
);

#[derive(Error, Display, Debug)]
#[display(
    fmt = "an event was not properly formatted (event id={}): {}",
    event_id,
    source
)]
pub struct IncorrectEventFormatting {
    pub source: serde_json::Error,
    pub event_id: i32,
}

impl_respondable_error!(
    IncorrectEventFormatting,
    INTERNAL_SERVER_ERROR,
    "INTERNAL_SERVER_ERROR"
);

#[derive(Error, Display, Debug)]
pub struct IncorrectEventOrdering;

impl_respondable_error!(
    IncorrectEventOrdering,
    INTERNAL_SERVER_ERROR,
    "INTERNAL_SERVER_ERROR"
);

#[derive(Error, Display, Debug)]
#[display(fmt = "the event type `{}` is not recognised", event_type)]
pub struct UnknownEventType {
    pub event_type: String,
}

impl_respondable_error!(
    UnknownEventType,
    INTERNAL_SERVER_ERROR,
    "INTERNAL_SERVER_ERROR"
);
