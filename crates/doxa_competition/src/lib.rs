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

    pub fn generate_configure_fn(&self) -> impl Fn(&mut web::ServiceConfig) + Clone {
        let competitions = Arc::new(self.competitions.clone());
        move |service| {
            //service.app_data(web::Data::new(context.clone()));
            for (name, competition) in competitions.iter() {
                let scoped_service_config =
                    service.service(web::scope(&format!("/competition/{}/", name)));
                competition.configure_routes(scoped_service_config);
            }
        }
    }

    pub async fn start(self) {
        for (_, competition) in self.competitions {
            competition
                .start_competition_manager(self.settings.clone())
                .await;
        }
    }
}
