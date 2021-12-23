use crate::schema::{game_events, game_participants, game_results, games};

use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};

pub use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Queryable)]
pub struct Game {
    pub id: i32,
    pub queued_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub competition: i32,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "games"]
pub struct InsertableGame {
    pub queued_at: DateTime<Utc>,
    pub competition: i32,
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "game_participants"]
pub struct GameParticipant {
    pub index: i32,
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
    pub game: i32,
    pub event_id: i32,
    pub event_timestamp: DateTime<Utc>,
    pub event_type: String,
    pub payload: JsonValue,
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "game_results"]
pub struct GameResult {
    pub agent: String,
    pub game: i32,
    pub result: i32,
}
