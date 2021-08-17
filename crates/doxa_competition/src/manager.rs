//! Contains methods related to the server-side management of competitions

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use doxa_core::tokio;

use crate::client::{validate_competition_name, BoxedCallback, Competition, Context};

pub struct CompetitionManager {
    upload_handlers: HashMap<String, BoxedCallback>,
    competitions: HashMap<String, Arc<dyn Competition>>,
}

pub struct CompetitionManagerBuilder {
    competitions: Vec<Box<dyn Competition>>,
    names: HashSet<String>,
}

impl CompetitionManagerBuilder {
    pub fn new() -> Self {
        CompetitionManagerBuilder {
            competitions: Vec::new(),
            names: HashSet::new(),
        }
    }

    /// Adds the competition to the builder.
    /// # Panics
    /// - If another competition has already registered a name this will panic.
    /// - If the name does not satisfy [`validate_competition_name`].
    pub fn add_competition<C: Into<Box<dyn Competition>>>(&mut self, competition: C) {
        let competition = competition.into();

        let name = competition.name();

        if !validate_competition_name(&name) {
            panic!(
                "The name `{}` does not satisfy the naming constraints",
                name
            );
        }

        if !self.names.insert(name) {
            panic!(
                "The name `{}` was already registered as a competition",
                competition.name()
            );
        }

        self.competitions.push(competition);
    }

    pub fn build(self) -> CompetitionManager {
        todo!();
    }
}

impl CompetitionManager {
    pub fn start(self) {}

    pub async fn next_upload(&self) {}
}
struct UploadEventManager {
    context: Context,
    upload_handlers: HashMap<String, (Arc<dyn Competition>, BoxedCallback)>,
    // connection:
}

impl UploadEventManager {
    fn new(
        context: Context,
        upload_handlers: HashMap<String, (Arc<dyn Competition>, BoxedCallback)>,
    ) -> Self {
        UploadEventManager {
            context,
            upload_handlers,
        }
    }

    fn start(self) {
        tokio::spawn(async move {
            loop {
                // self.connection.next() ...
            }
        });
    }
}

/*
use crate::client::Competition;

pub struct CompetitionManager {
    competitions: Vec<Competition>,
}

impl CompetitionManager {
    pub fn new() -> Self {
        CompetitionManager {
            competitions: Vec::new(),
        }
    }

    pub fn add_competition(&mut self, competition: Competition) {
        self.competitions.push(competition);
    }

    pub fn start(self) {

    }
}
*/
