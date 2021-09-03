use derive_more::{Display, Error, From};
use doxa_core::{deadpool_lapin, lapin, tokio::task::JoinError};
use doxa_db::diesel::r2d2;

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
}
