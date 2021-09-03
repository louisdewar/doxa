//! Contains methods related to the server-side management of competitions

use std::{collections::HashMap, sync::Arc};

use doxa_core::actix_web::{self, web};
use doxa_mq::Connection as MQConnection;
use futures::future::{join, join_all};

use crate::{
    client::{validate_competition_name, Competition, CompetitionInner, Context},
    Settings,
};

use self::{executor::ExecutionManager, upload::UploadEventManager};

pub(crate) mod executor;
mod upload;

pub struct CompetitionManager<T: Competition> {
    competition: Arc<T>,
    settings: Arc<Settings>,
}

impl<T: Competition> CompetitionManager<T> {
    /// Spawns tasks required for managing the competition
    pub async fn start(competition: Arc<T>, settings: Arc<Settings>) {
        // TODO: decide whether or not this needs to be async + Error
        let manager = CompetitionManager {
            competition,
            settings,
        };

        let upload_manager =
            UploadEventManager::new(manager.settings.clone(), manager.competition.clone());
        let execution_manager =
            ExecutionManager::<T::GameClient>::new(manager.settings, T::COMPETITION_NAME);

        join(upload_manager.start(), execution_manager.start()).await;
    }
}
