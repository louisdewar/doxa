use std::sync::Arc;

use doxa_core::{
    lapin::options::BasicAckOptions,
    tokio,
    tracing::{event, span, Level},
    tracing_futures::Instrument,
};

use crate::Settings;
use doxa_mq::model::UploadEvent;

use futures::StreamExt;

use crate::client::{Competition, Context};

pub(super) struct UploadEventManager<C: Competition> {
    settings: Arc<Settings>,
    competition: Arc<C>,
}

impl<C: Competition> UploadEventManager<C> {
    pub fn new(settings: Arc<Settings>, competition: Arc<C>) -> Self {
        UploadEventManager {
            settings,
            competition,
        }
    }

    pub async fn start(self) {
        let connection = self
            .settings
            .mq_pool
            .get()
            .await
            .expect("Failed to get MQ connection");

        let mut consumer =
            doxa_mq::action::get_upload_event_consumer(&connection, C::COMPETITION_NAME)
                .await
                .unwrap();

        let span = span!(
            Level::INFO,
            "upload event listener",
            competition = C::COMPETITION_NAME
        );
        let future = async move {
            let mut context =
                Context::new(self.settings.mq_pool.clone(), self.settings.pg_pool.clone());
            // TODO: In future just log error and retry with timeout
            while let Some(message) = consumer.next().await {
                let (_, delivery) = message.expect("Error connecting to MQ");

                let upload: UploadEvent = doxa_mq::action::deserialize(&delivery.data)
                    .expect("Improperly formatted message");
                let agent_id = upload.agent.clone();
                event!(Level::INFO, %agent_id, "received upload event for agent");

                if let Err(error) = self.competition.on_upload(&mut context, upload).await {
                    event!(Level::ERROR, %error, %agent_id, "on_upload failed for agent");
                    continue;
                }

                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("Failed to acknowledge MQ");
            }
        };

        tokio::spawn(future.instrument(span));
    }
}
