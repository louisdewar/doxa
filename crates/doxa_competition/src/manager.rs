//! Contains methods related to the server-side management of competitions

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use doxa_core::{
    actix_web::{self, web, Scope},
    tokio,
};
use doxa_mq::Connection as MQConnection;

use crate::client::{validate_competition_name, BoxedCallback, Competition, Context};

use self::upload::UploadEventManager;

mod upload;

pub struct Settings {
    rabbit_mq_address: String,
}

pub struct CompetitionManager {
    //upload_handlers: HashMap<String, BoxedCallback>,
    competitions: HashMap<String, Arc<dyn Competition>>,
}

pub struct CompetitionManagerBuilder {
    competitions: HashMap<String, Arc<dyn Competition>>,
}

impl CompetitionManagerBuilder {
    pub fn new() -> Self {
        CompetitionManagerBuilder {
            competitions: Default::default(),
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
            .insert(name.clone(), competition)
            .is_some()
        {
            panic!(
                "The name `{}` was already registered as a competition",
                &name
            );
        }
    }

    pub fn build(self) -> CompetitionManager {
        CompetitionManager {
            competitions: self.competitions,
        }
    }
}

impl CompetitionManager {
    fn generate_configure_fn(
        &self,
        context: Context,
    ) -> impl Fn(&mut actix_web::web::ServiceConfig) + Clone {
        let competitions = Arc::new(self.competitions.clone());
        move |service| {
            service.app_data(web::Data::new(context.clone()));
            for (name, competition) in competitions.iter() {
                let scoped_service_config =
                    service.service(web::scope(&format!("/competition/{}/", name)));
                competition.configure_routes(scoped_service_config);
            }
        }
    }

    /// Configures actix routes and then spawns up tasks for handling events such as uploads,
    /// results and timers.
    ///
    /// This returns a function for configuring actix routes
    pub fn start(
        self,
        rabbit_mq: MQConnection,
    ) -> impl Fn(&mut actix_web::web::ServiceConfig) + Clone {
        let context = Context {};

        let upload_manager =
            UploadEventManager::new(context.clone(), self.competitions.clone(), rabbit_mq);
        upload_manager.start();
        //tokio::spawn(async move {});
        // let addr =
        // std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
        self.generate_configure_fn(context)
    }
}
