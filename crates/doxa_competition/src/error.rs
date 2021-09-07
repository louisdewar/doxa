use derive_more::{Display, Error, From};
use doxa_core::{actix_web, deadpool_lapin, impl_respondable_error, lapin, tokio::task::JoinError};
use doxa_db::{diesel::r2d2, DieselError};

#[derive(From, Error, Display, Debug)]
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
}

#[derive(From, Error, Display, Debug)]
/// Errors that occur from the competition managers
pub enum CompetitionManagerError {
    #[display(fmt = "this competition has not been registered in the database")]
    CompetitionNotFound,
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
