use std::{collections::HashMap, sync::Arc};

use client::{validate_competition_name, Competition, CompetitionInner};

use doxa_core::actix_web::web;

pub mod client;
pub mod context;
pub mod error;
pub mod hello_world;
pub mod manager;
pub mod route;
pub mod settings;

pub use settings::Settings;

use doxa_core::tracing::{error, info};

pub struct CompetitionSystem {
    competitions: HashMap<String, CompetitionRecord>,
}

#[derive(Clone)]
struct CompetitionRecord {
    competition: Arc<dyn CompetitionInner>,
    executor_permits: usize,
}

impl CompetitionSystem {
    pub fn new() -> Self {
        CompetitionSystem {
            competitions: Default::default(),
        }
    }

    /// Adds the competition to the builder.
    ///
    /// `executor_permits` the number of simultaneous executions for this competition.
    ///
    /// # Panics
    /// - If another competition has already registered a name this will panic.
    /// - If the name does not satisfy [`validate_competition_name`].
    pub fn add_competition<C: Competition>(&mut self, competition: C, executor_permits: usize) {
        assert!(
            executor_permits > 0,
            "competition must have at least one permit"
        );
        let competition = Arc::new(competition);

        let name = competition.name();

        if !validate_competition_name(name) {
            panic!(
                "The name `{}` does not satisfy the naming constraints",
                name
            );
        }

        if self
            .competitions
            .insert(
                name.to_string(),
                CompetitionRecord {
                    competition,
                    executor_permits,
                },
            )
            .is_some()
        {
            panic!(
                "The name `{}` was already registered as a competition",
                &name
            );
        }
    }

    pub async fn start(self, settings: Arc<Settings>) -> impl Fn(&mut web::ServiceConfig) + Clone {
        let mut competitions = Vec::with_capacity(self.competitions.len());
        // TODO: try join all
        for (competition_name, record) in self.competitions {
            match record
                .competition
                .clone()
                .start_competition_manager(settings.clone(), record.executor_permits)
                .await
            {
                Err(error) => {
                    error!(%competition_name, %error, error_debug=?error, "failed to start competition manager")
                }
                Ok(competition_id) => {
                    // TODO: timer for startup
                    info!(%competition_name, "started competition manager");
                    competitions.push((competition_name, record, competition_id));
                }
            }
        }

        let settings = settings.clone();

        move |service| {
            for (name, record, competition_id) in competitions.iter() {
                service.service(web::scope(&format!("/competition/{}", name)).configure(
                    |config| {
                        record
                            .competition
                            .configure_routes(config, &settings, *competition_id)
                    },
                ));
            }
        }
    }
}

impl Default for CompetitionSystem {
    fn default() -> Self {
        Self::new()
    }
}
