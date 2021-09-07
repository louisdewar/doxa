use crate::schema::{game_events, game_participants, games};

use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};

pub use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Queryable)]
pub struct Game {
    pub id: i32,
    pub start_time: DateTime<Utc>,
    pub complete_time: Option<DateTime<Utc>>,
    pub competition: i32,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "games"]
pub struct InsertableGame {
    pub start_time: DateTime<Utc>,
    pub competition: i32,
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "game_participants"]
pub struct GameParticipant {
    pub agent: String,
    pub game: i32,
}

#[derive(Debug, Clone, Queryable)]
/// Returns not only the agent ID but also the user's ID.
/// (this will be run as part of a JOIN query).
pub struct GameParticipantUser {
    pub agent: String,
    pub user: i32,
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "game_events"]
pub struct GameEvent {
    pub event_id: i32,
    pub game: i32,
    pub event_timestamp: DateTime<Utc>,
    pub event_type: String,
    pub payload: JsonValue,
}
