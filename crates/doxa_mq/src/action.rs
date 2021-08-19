use doxa_core::lapin::options::BasicConsumeOptions;
use doxa_core::lapin::{self, Connection, Consumer};

use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    publisher_confirm::PublisherConfirm,
    types::FieldTable,
    BasicProperties, Channel,
};

use crate::model::UploadEvent;

pub use bincode::{deserialize, serialize};

pub fn upload_queue_name(competition_name: &str) -> String {
    format!("uploadevent.{}", competition_name)
}

pub async fn declare_upload_queue(
    channel: &Channel,
    competition_name: &str,
) -> Result<lapin::Queue, lapin::Error> {
    channel
        .queue_declare(
            &upload_queue_name(competition_name),
            QueueDeclareOptions {
                // It may not need to be durable, as part of the startup proceedure the system
                // could go through agents that have no queued games or there could be a field on
                // agent which determines if it's been processed or not (this probably isn't a good
                // idea as then what's the point of rabbitmq)
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
}

/// This declares the correct queue then submits the event.
pub async fn emit_upload_event(
    conn: &Connection,
    upload_event: &UploadEvent,
) -> Result<PublisherConfirm, lapin::Error> {
    let channel = conn.create_channel().await?;
    declare_upload_queue(&channel, &upload_event.competition).await?;

    channel
        .basic_publish(
            "",
            &upload_queue_name(&upload_event.competition),
            BasicPublishOptions::default(),
            serialize(upload_event).unwrap(),
            BasicProperties::default(),
        )
        .await
}

/// This declares the correct queue then returns the consumer which can yield messages.
pub async fn get_upload_event_consumer(
    conn: &Connection,
    competition_name: &str,
) -> Result<Consumer, lapin::Error> {
    let channel = conn.create_channel().await?;
    declare_upload_queue(&channel, competition_name).await?;

    channel
        .basic_consume(
            &upload_queue_name(competition_name),
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
}
