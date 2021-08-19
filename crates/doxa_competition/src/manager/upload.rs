use std::{collections::HashMap, sync::Arc};

use doxa_core::{lapin::options::BasicAckOptions, tokio};

use doxa_mq::{lapin, model::UploadEvent, tokio_amqp};

use futures::StreamExt;
use lapin::{Connection, ConnectionProperties};
use tokio_amqp::LapinTokioExt;

use crate::client::{BoxedCallback, Competition, Context};

pub(super) struct UploadEventManager {
    context: Context,
    upload_handlers: HashMap<String, Arc<dyn Competition>>,
    connection: Connection,
}

impl UploadEventManager {
    pub fn new(
        context: Context,
        upload_handlers: HashMap<String, Arc<dyn Competition>>,
        connection: Connection,
    ) -> Self {
        UploadEventManager {
            context,
            upload_handlers,
            connection,
        }
    }

    pub fn start(self) {
        tokio::spawn(async move {
            for (name, competition) in self.upload_handlers {
                let mut context = self.context.clone();
                let mut consumer =
                    doxa_mq::action::get_upload_event_consumer(&self.connection, &name)
                        .await
                        .unwrap();
                tokio::spawn(async move {
                    // TODO: In future just log error and retry with timeout
                    while let Some(message) = consumer.next().await {
                        let (_, delivery) = message.expect("Error connecting to MQ");

                        let upload: UploadEvent = doxa_mq::action::deserialize(&delivery.data)
                            .expect("Improperly formatted message");

                        competition.on_upload(&mut context, upload).await;
                        delivery
                            .ack(BasicAckOptions::default())
                            .await
                            .expect("Failed to acknowledge MQ");
                    }
                });
            }
        });
    }
}
