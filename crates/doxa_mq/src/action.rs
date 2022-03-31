use doxa_core::lapin::options::BasicConsumeOptions;
use doxa_core::lapin::{self, Connection, Consumer};

use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    publisher_confirm::PublisherConfirm,
    types::FieldTable,
    BasicProperties, Channel,
};
use serde::Serialize;

use crate::model::{ActivationEvent, MatchRequest};

pub use bincode::Error as BincodeError;
pub use bincode::{deserialize, serialize};

pub fn activation_queue_name(competition_name: &str) -> String {
    format!("activationevent.{}", competition_name)
}

pub fn game_event_queue_name(competition_name: &str) -> String {
    format!("gameevent.{}", competition_name)
}

pub fn match_request_queue_name(competition_name: &str, execution_profile: &str) -> String {
    format!("matchrequest.{}.{}", competition_name, execution_profile)
}

pub async fn declare_activation_queue(
    channel: &Channel,
    competition_name: &str,
) -> Result<lapin::Queue, lapin::Error> {
    declare(channel, &activation_queue_name(competition_name), true).await
}

pub async fn declare_game_event_queue(
    channel: &Channel,
    competition_name: &str,
) -> Result<lapin::Queue, lapin::Error> {
    declare(channel, &game_event_queue_name(competition_name), true).await
}

pub async fn declare_match_request_queue(
    channel: &Channel,
    competition_name: &str,
    execution_profile: &str,
) -> Result<lapin::Queue, lapin::Error> {
    declare(
        channel,
        &match_request_queue_name(competition_name, execution_profile),
        true,
    )
    .await
}

async fn declare(
    channel: &Channel,
    queue_name: &str,
    durable: bool,
) -> Result<lapin::Queue, lapin::Error> {
    channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions {
                // It may not need to be durable, as part of the startup proceedure the system
                // could go through agents that have no queued games or there could be a field on
                // agent which determines if it's been processed or not (this probably isn't a good
                // idea as then what's the point of rabbitmq)
                durable,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
}

pub async fn publish(
    channel: &Channel,
    queue_name: &str,
    payload: Vec<u8>,
) -> Result<PublisherConfirm, lapin::Error> {
    channel
        .basic_publish(
            "",
            queue_name,
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default(),
        )
        .await
}

async fn consume(channel: &Channel, queue_name: &str) -> Result<Consumer, lapin::Error> {
    channel
        .basic_consume(
            queue_name,
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
}

/// This declares the correct queue then submits the event.
pub async fn emit_activation_event(
    conn: &Connection,
    upload_event: &ActivationEvent,
) -> Result<PublisherConfirm, lapin::Error> {
    let channel = conn.create_channel().await?;
    declare_activation_queue(&channel, &upload_event.competition).await?;

    publish(
        &channel,
        &activation_queue_name(&upload_event.competition),
        serialize(upload_event).unwrap(),
    )
    .await
}

/// This declares the correct queue then returns the consumer which can yield messages.
pub async fn get_activation_event_consumer(
    conn: &Connection,
    competition_name: &str,
) -> Result<Consumer, lapin::Error> {
    let channel = conn.create_channel().await?;
    declare_activation_queue(&channel, competition_name).await?;

    consume(&channel, &activation_queue_name(competition_name)).await
}

pub async fn emit_match_request<T: Serialize>(
    conn: &Connection,
    match_request: &MatchRequest<T>,
    competition: &str,
    execution_profile: &str,
) -> Result<PublisherConfirm, lapin::Error> {
    let channel = conn.create_channel().await?;
    declare_match_request_queue(&channel, competition, execution_profile).await?;

    publish(
        &channel,
        &match_request_queue_name(competition, execution_profile),
        serialize(match_request).unwrap(),
    )
    .await
}

pub async fn get_match_request_consumer(
    conn: &Connection,
    competition_name: &str,
    execution_profile: &str,
) -> Result<Consumer, lapin::Error> {
    let channel = conn.create_channel().await?;
    declare_match_request_queue(&channel, competition_name, execution_profile).await?;

    consume(
        &channel,
        &match_request_queue_name(competition_name, execution_profile),
    )
    .await
}

/// This declares the correct queue then returns the consumer which can yield messages.
pub async fn get_game_event_consumer(
    conn: &Connection,
    competition_name: &str,
) -> Result<Consumer, lapin::Error> {
    let channel = conn.create_channel().await?;
    declare_game_event_queue(&channel, competition_name).await?;

    consume(&channel, &game_event_queue_name(competition_name)).await
}
// pub async fn emit_game_event<T: Serialize>(
//     conn: &Connection,
//     game_event: &GameEvent<T>,
//     competition: &str,
// ) -> Result<PublisherConfirm, lapin::Error> {
//     let channel = conn.create_channel().await?;
//     declare_game_event_queue(&channel, competition).await?;
//
//     publish(
//         &channel,
//         &game_event_queue_name(competition),
//         serialize(game_event).unwrap(),
//     )
//     .await
// }

/// Declares the queue then returns the channel where it is safe to use publish later
pub async fn game_event_channel(
    conn: &Connection,
    competition: &str,
) -> Result<Channel, lapin::Error> {
    let channel = conn.create_channel().await?;
    declare_game_event_queue(&channel, competition).await?;
    Ok(channel)
}
