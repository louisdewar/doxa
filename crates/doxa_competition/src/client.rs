//! Contains methods useful for implementing a new competition

use std::sync::Arc;

use doxa_auth::limiter::{GenericLimiter, LimiterConfig};
use doxa_core::actix_web::{self, web};

use crate::{
    error::{CompetitionManagerError, ContextError},
    manager::CompetitionManager,
    route::{self, limits::CompetitionLimits},
    Settings,
};

pub use crate::context::Context;
pub use async_trait::async_trait;
pub use doxa_auth::limiter;
pub use doxa_db::model::storage::AgentUpload;
pub use doxa_executor::client::{
    ForfeitError, GameClient, GameContext, GameError, Mount, VMBackend,
};
pub use doxa_mq::model::{ActivationEvent, GameEvent};
pub use serde_json;

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

    !name.chars().any(|c| !c.is_ascii_lowercase())
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
    async fn startup(&self, context: &Context<Self>) -> Result<(), ContextError>;

    /// Sets up routes with an automatic prefix of `/competition/{competition_name}`.
    /// This may run multiple times.
    ///
    /// This is the appropriate place to insert `app_data` such as configs.
    ///
    ///
    /// This method does not configure the system routes (ones that begin with `_`), those have
    /// their own methods.
    ///
    /// The default does not configure any routes (NOTE: system routes are configured
    /// independently).
    fn configure_routes(&self, _service: &mut actix_web::web::ServiceConfig) {}

    /// Runs whenever an agent has been activated.
    ///
    /// TODO: consider the guarantees regarding the ordering of agent activated / deactivated and
    /// how to make then stronger - mutex uploads might do the trick + rate limiting of at least a
    /// few seconds from completion of an upload
    async fn on_agent_activated(
        &self,
        context: &Context<Self>,
        agent: AgentUpload,
    ) -> Result<(), ContextError>;

    /// Runs whenever an agent has been deactivated.
    async fn on_agent_deactivated(
        &self,
        context: &Context<Self>,
        agent: AgentUpload,
    ) -> Result<(), ContextError>;

    /// Whenever a game ends without an error.
    /// The event will already have been stored in the db.
    ///
    /// The default implementation does nothing
    async fn on_game_end(
        &self,
        _context: &Context<Self>,
        _event: GameEvent<()>,
    ) -> Result<(), ContextError> {
        Ok(())
    }

    /// Returns a new instance of the [`Self::GameClient`] for this competition.
    /// If there are configuration parameters stored under `Self` then this is the place to give
    /// them to the game client.
    fn build_game_client(&self) -> Self::GameClient;

    /// Whenever ends with a fatal error during **runtime** (i.e. any errors that occur during the
    /// `run` method as part of `GameClient`.
    /// Errors that occur at startup e.g. downloading the agent/starging the VM are not currently
    /// recorded here.
    /// The event will already have been stored in the db.
    ///
    /// This includes both the `GameContextError` and the errors the the game client emits.
    ///
    /// The default implementation does nothing.
    async fn on_game_error(
        &self,
        _context: &Context<Self>,
        _event: GameEvent<GameError<<Self::GameClient as GameClient>::Error>>,
    ) -> Result<(), ContextError> {
        Ok(())
    }

    /// This only includes events emitted by the game client (not system messages).
    /// The event will already have been stored in the db.
    async fn on_game_event(
        &self,
        context: &Context<Self>,
        event: GameEvent<<Self::GameClient as GameClient>::GameEvent>,
    ) -> Result<(), ContextError>;

    /// This function should register the `/_game/{game_id}/...` routes. It provides a default implementation
    /// that uses Self::event_filter. If you need something more advanced you can overwrite this.
    /// If you don't want these routes you can redefine this as an empty function.
    fn configure_game_routes(&self, service: &mut actix_web::web::ServiceConfig) {
        service.route("_game/{game_id}", web::get().to(route::game::game::<Self>));

        service.route(
            "_game/{game_id}/events",
            web::get().to(route::game::game_events::<Self>),
        );

        service.route(
            "_game/{game_id}/players",
            web::get().to(route::game::game_players::<Self>),
        );

        service.route(
            "_game/{game_id}/result/{agent_id}",
            web::get().to(route::game::game_result_agent::<Self>),
        );

        service.route(
            "_game/{game_id}/cancelled",
            web::get().to(route::game::game_cancelled::<Self>),
        );
    }

    /// This function should register the `/_agent/{agent_id}/...` routes.
    fn configure_agent_routes(&self, service: &mut actix_web::web::ServiceConfig) {
        service.route(
            "_agent/{agent_id}/games",
            web::get().to(route::agent::agent_games::<Self>),
        );

        service.route(
            "_agent/{agent_id}/score",
            web::get().to(route::agent::agent_score_primary::<Self>),
        );

        service.route(
            "_agent/{agent_id}/score/{leaderboard}",
            web::get().to(route::agent::agent_score::<Self>),
        );

        service.route(
            "_agent/{agent_id}/reactivate",
            web::post().to(route::agent::reactivate_agent::<Self>),
        );

        service.route(
            "_agent/{agent_id}/deactivate",
            web::post().to(route::agent::deactivate_agent::<Self>),
        );

        service.route(
            "_agent/{agent_id}/activate",
            web::post().to(route::agent::activate_agent::<Self>),
        );

        service.route(
            "_agent/{agent_id}/owner",
            web::get().to(route::agent::agent_owner::<Self>),
        );
    }

    /// This function should register the `/_user/{username}/...` routes.
    fn configure_user_routes(&self, service: &mut actix_web::web::ServiceConfig) {
        service.route(
            "_user/{username}/agents",
            web::get().to(route::user::user_agents::<Self>),
        );

        service.route(
            "_user/{username}/active_agent",
            web::get().to(route::user::user_active_agent::<Self>),
        );

        // service.route(
        //     "_user/{username}/high_score",
        //     web::get().to(route::user::user_high_score::<Self>),
        // );

        service.route(
            "_user/{username}/score",
            web::get().to(route::user::user_score_primary::<Self>),
        );

        service.route(
            "_user/{username}/score/{leaderboard}",
            web::get().to(route::user::user_score::<Self>),
        );

        service.route(
            "_user/{username}/active_games",
            web::get().to(route::user::user_active_games::<Self>),
        );

        service.route(
            "_user/{username}/reactivate_active_agent",
            web::post().to(route::user::reactivate_agent::<Self>),
        );

        service.route(
            "_user/{username}/deactivate_active_agent",
            web::post().to(route::user::deactivate_agent::<Self>),
        );

        // TODO:
        // service.route(
        //     "_user/{username}/rank",
        //     web::get().to(route::user_rank::<Self>),
        // );
    }

    /// This function registers the `/_leaderboard/...` routes.
    ///
    /// If you want to customise this or disable this you can overwrite this function.
    fn configure_leaderboard_routes(&self, service: &mut actix_web::web::ServiceConfig) {
        service.route(
            "_leaderboard/active",
            web::get().to(route::leaderboard::active_leaderboard_primary::<Self>),
        );

        service.route(
            "_leaderboard/active/{leaderboard}",
            web::get().to(route::leaderboard::active_leaderboard::<Self>),
        );
    }

    /// This function registers the `/_upload`.
    ///
    /// If you want to customise this or disable this you can overwrite this function.
    fn configure_upload_routes(&self, service: &mut actix_web::web::ServiceConfig) {
        service.route("_upload", web::post().to(route::upload::upload::<Self>));
    }

    /// Builds a limiter to use for this competition for uploads and activations.
    /// Admins can ignore this limit.
    ///
    /// If you want to define your own limiter you should use the provided key.
    ///
    /// The default is `doxa_storage::limits::upload_attempts_limiter`
    fn upload_limiter(&self, key: String) -> LimiterConfig {
        doxa_storage::limits::default_upload_attempts_limiter(key)
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
    /// This call both the `configure_routes` and the specific route configuration functions.
    ///
    /// This also adds the context `app_data`.
    fn configure_routes(
        &self,
        service: &mut actix_web::web::ServiceConfig,
        settings: &Settings,
        competition_id: i32,
    );

    fn build_competition_limiter(&self, generic_limiter: Arc<GenericLimiter>) -> CompetitionLimits;

    // A temporary route to keep the old storage upload path working for older versions of the CLI
    // This cannot be added in `configure_routes` because the routes are prefixed.
    fn configure_upload_route(
        &self,
        limits: web::Data<CompetitionLimits>,
        name: String,
        service: &mut actix_web::web::ServiceConfig,
    );

    async fn start_competition_manager(
        self: Arc<Self>,
        settings: Arc<Settings>,
        executor_permits: usize,
    ) -> Result<i32, CompetitionManagerError>;

    fn name(&self) -> &'static str;
}

#[async_trait]
impl<T: Competition> CompetitionInner for T {
    fn configure_routes(
        &self,
        service: &mut actix_web::web::ServiceConfig,
        settings: &Settings,
        competition_id: i32,
    ) {
        service.app_data(web::Data::new(Context::<T>::new(
            settings.mq_pool.clone(),
            settings.pg_pool.clone(),
            competition_id,
        )));
        Competition::configure_game_routes(self, service);
        Competition::configure_agent_routes(self, service);
        Competition::configure_user_routes(self, service);
        Competition::configure_leaderboard_routes(self, service);
        Competition::configure_upload_routes(self, service);

        Competition::configure_routes(self, service);
    }

    async fn start_competition_manager(
        self: Arc<Self>,
        settings: Arc<Settings>,
        executor_permits: usize,
    ) -> Result<i32, CompetitionManagerError> {
        CompetitionManager::start(self, settings, executor_permits).await
    }

    fn configure_upload_route(
        &self,
        limits: web::Data<CompetitionLimits>,
        name: String,
        service: &mut actix_web::web::ServiceConfig,
    ) {
        service.service(
            web::resource(&format!("/storage/upload/{}", name))
                .app_data(limits)
                .route(web::post().to(route::upload::upload::<Self>)),
        );
    }

    fn build_competition_limiter(&self, generic_limiter: Arc<GenericLimiter>) -> CompetitionLimits {
        let activations = Competition::upload_limiter(
            self,
            format!("ACTIVATION_LIMIT_{}", <T as Competition>::COMPETITION_NAME),
        );

        CompetitionLimits::new(generic_limiter, activations)
    }

    fn name(&self) -> &'static str {
        T::COMPETITION_NAME
    }
}
