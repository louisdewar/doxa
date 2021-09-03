//! Contains methods useful for implementing a new competition

use std::{future::Future, pin::Pin, sync::Arc};

use async_trait::async_trait;

use doxa_core::{actix_web, lapin};
use doxa_executor::client::GameClient;
use doxa_mq::model::UploadEvent;

use crate::{
    error::ContextError,
    manager::{executor::ExecutionManager, CompetitionManager},
    Settings,
};

pub use crate::context::Context;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
pub type BoxedCallback = Box<dyn Callback>;

/// Returns true if the competition name is valid.
/// Competition names must be entirely consisting of ASCII lowercase letters [a-z]
/// or hyphens '-'.
/// No other characters are allowed.
/// They length must be greater than 3 characters and less than 20.
///
/// The name must also be unique but this function will not check that.
pub fn validate_competition_name(name: &str) -> bool {
    if !(name.len() > 3 && name.len() < 20) {
        return false;
    }

    name.chars().find(|c| !c.is_ascii_lowercase()).is_none()
}

pub trait Callback: 'static {
    fn call(&self) -> BoxFuture<'static, ()>;
}

impl<F: Future<Output = ()> + Send + 'static, T: Fn() -> F + 'static> Callback for T {
    fn call(&self) -> BoxFuture<'static, ()> {
        Box::pin(self())
    }
}

impl<T: Callback> From<T> for Box<dyn Callback> {
    fn from(callback: T) -> Self {
        Box::new(callback)
    }
}

#[async_trait]
pub trait Competition: 'static + Send + Sync {
    type GameClient: GameClient;

    /// See [`validate_competition_name`] for more info regarding allowed names.
    const COMPETITION_NAME: &'static str;

    // Maybe &mut self could be enforced as startup happens before everything else
    // could also be the case that startup returns Self.
    /// Runs exactly once at startup before all other functions
    // Maybe add StartupContext for things such as registering timers.
    async fn startup(&self, context: &mut Context<Self>) -> Result<(), ContextError>;

    /// Sets up routes with an automatic prefix of `/competition/{competition_name}`.
    /// This may run multiple times.
    ///
    /// This is the appropriate place to insert `app_data` such as configs.
    fn configure_routes(&self, service: &mut actix_web::web::ServiceConfig);

    /// Runs whenever a new agent has been successfull uploaded.
    /// TODO: upload info
    async fn on_upload(
        &self,
        context: &mut Context<Self>,
        upload_event: UploadEvent,
    ) -> Result<(), ContextError>;

    /// Runs whenever the result of an execution (commonly called a match) has been completed.
    /// TODO: execution info
    /// maybe have a `.save(conn)` method to save to the database
    async fn on_execution_result(&self, context: &mut Context<Self>) -> Result<(), ContextError>;
}

/// A trait that is similar to Competition except it supports dynamic dispatch.
#[async_trait]
pub(crate) trait CompetitionInner: 'static + Send + Sync {
    fn configure_routes(&self, service: &mut actix_web::web::ServiceConfig);

    // async fn on_upload(&self, context: &mut Context, upload_event: UploadEvent);

    // async fn on_execution_result(&self, context: &mut Context);

    // /// Starts the various competition management systems, e.g. upload and executor
    // async fn start_execution_manager(
    //     &self,
    //     connection: &lapin::Connection,
    //     executor_settings: Arc<doxa_executor::Settings>,
    // );

    async fn start_competition_manager(self: Arc<Self>, settings: Arc<Settings>);

    fn name(&self) -> &'static str;
}

#[async_trait]
impl<T: Competition> CompetitionInner for T {
    fn configure_routes(&self, service: &mut actix_web::web::ServiceConfig) {
        Competition::configure_routes(self, service)
    }

    // async fn on_upload(&self, context: &mut Context, upload_event: UploadEvent) {
    //     Competition::on_upload(self, context, upload_event).await
    // }

    // async fn on_execution_result(&self, context: &mut Context) {
    //     Competition::on_execution_result(self, context).await
    // }

    async fn start_competition_manager(self: Arc<Self>, settings: Arc<Settings>) {
        // TODO: decide whether or not this needs to be async + Error
        CompetitionManager::start(self, settings).await;
    }

    // // Maybe take in Arc self so that it can be used in the manager/game client?
    // /// Start the execution manager then moves itself onto it's own thread for listening to
    // /// messages after setup
    // async fn start_execution_manager(&self, settings: Arc<Settings>) {
    //     let manager = ExecutionManager::<T::GameClient>::new(settings, self.name()).await;
    //     manager.start(connection, executor_settings).await;
    // }

    fn name(&self) -> &'static str {
        T::COMPETITION_NAME
    }
}
