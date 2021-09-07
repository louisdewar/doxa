//! Contains methods useful for implementing a new competition

use std::sync::Arc;

use async_trait::async_trait;

use doxa_core::actix_web::{self, web};
use doxa_executor::client::GameClient;
use doxa_mq::model::UploadEvent;

use crate::{
    error::{CompetitionManagerError, ContextError},
    manager::CompetitionManager,
    Settings,
};

pub use crate::context::Context;

pub mod route;

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
    ///
    /// This does not need to call `configure_event_routes` this will happen automatically.
    /// If you don't want that route configured you should override that method.
    fn configure_routes(&self, service: &mut actix_web::web::ServiceConfig);

    /// Runs whenever a new agent has been successfull uploaded.
    /// TODO: upload info
    async fn on_upload(
        &self,
        context: &mut Context<Self>,
        upload_event: UploadEvent,
    ) -> Result<(), ContextError>;

    /// This only includes events emitted by the game client (not system messages).
    /// The event will already have been stored in the db
    async fn on_game_event(
        &self,
        context: &mut Context<Self>,
        event: <Self::GameClient as GameClient>::GameEvent,
    ) -> Result<(), ContextError>;

    /// This function should register the `/_events` route. It provides a default implementation
    /// that uses Self::event_filter. If you need something more advanced you can overwrite this.
    /// If you don't want the `/_events` route you can define this as an empty function.
    fn configure_event_routes(&self, service: &mut actix_web::web::ServiceConfig) {
        service.route(
            "/_events/{game_id}",
            web::get().to(route::default_events_route::<Self>),
        );
    }

    /// Filter maps the events before sending them to a user.
    /// If the user was a participant in the current game then the ID if their agent will be
    /// provided in the agent field (this is the 0 indexed id).
    /// This defaults to always returning None which will mean no events are sent to the user.
    fn event_filter(
        _game_event: <Self::GameClient as GameClient>::GameEvent,
        _is_admin: bool,
        _agent: Option<usize>,
    ) -> Option<serde_json::Value> {
        None
    }
}

/// A trait that is similar to Competition except it supports dynamic dispatch.
#[async_trait]
pub(crate) trait CompetitionInner: 'static + Send + Sync {
    /// This call both the `configure_routes` and `configure_event_routes` functions.
    fn configure_routes(&self, service: &mut actix_web::web::ServiceConfig);

    async fn start_competition_manager(
        self: Arc<Self>,
        settings: Arc<Settings>,
    ) -> Result<(), CompetitionManagerError>;

    fn name(&self) -> &'static str;
}

#[async_trait]
impl<T: Competition> CompetitionInner for T {
    fn configure_routes(&self, service: &mut actix_web::web::ServiceConfig) {
        Competition::configure_event_routes(self, service);
        Competition::configure_routes(self, service);
    }

    async fn start_competition_manager(
        self: Arc<Self>,
        settings: Arc<Settings>,
    ) -> Result<(), CompetitionManagerError> {
        CompetitionManager::start(self, settings).await
    }

    fn name(&self) -> &'static str {
        T::COMPETITION_NAME
    }
}
