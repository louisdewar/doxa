use std::{collections::HashMap, sync::Arc};

use client::{validate_competition_name, Competition, CompetitionInner};

use doxa_core::actix_web::web;

pub mod client;
pub mod context;
pub mod error;
pub mod hello_world;
pub mod manager;
pub mod settings;

pub use settings::Settings;

use doxa_core::tracing::{error, info};

pub struct CompetitionSystem {
    competitions: HashMap<String, Arc<dyn CompetitionInner>>,
    settings: Arc<Settings>,
}

impl CompetitionSystem {
    pub fn new(settings: Arc<Settings>) -> Self {
        CompetitionSystem {
            competitions: Default::default(),
            settings,
        }
    }

    /// Adds the competition to the builder.
    /// # Panics
    /// - If another competition has already registered a name this will panic.
    /// - If the name does not satisfy [`validate_competition_name`].
    pub fn add_competition<C: Competition>(&mut self, competition: C) {
        let competition = Arc::new(competition);

        let name = competition.name();

        if !validate_competition_name(&name) {
            panic!(
                "The name `{}` does not satisfy the naming constraints",
                name
            );
        }

        if self
            .competitions
            .insert(name.to_string(), competition)
            .is_some()
        {
            panic!(
                "The name `{}` was already registered as a competition",
                &name
            );
        }
    }

    pub async fn start(self) -> impl Fn(&mut web::ServiceConfig) + Clone {
        let mut competitions = Vec::with_capacity(self.competitions.len());
        // TODO: try join all
        for (competition_name, competition) in self.competitions {
            match competition
                .clone()
                .start_competition_manager(self.settings.clone())
                .await
            {
                Err(error) => {
                    error!(%competition_name, %error, error_debug=?error, "failed to start competition manager")
                }
                Ok(competition_id) => {
                    // TODO: timer for startup
                    info!(%competition_name, "started competition manager");
                    competitions.push((competition_name, competition, competition_id));
                }
            }
        }

        let settings = self.settings.clone();

        move |service| {
            for (name, competition, competition_id) in competitions.iter() {
                service.service(web::scope(&format!("/competition/{}", name)).configure(
                    |config| competition.configure_routes(config, &settings, *competition_id),
                ));
            }
        }
    }
}
