use doxa_core::chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GameEventsResponse {
    pub events: Vec<GameEventResponse>,
}

#[derive(Serialize, Debug)]
pub struct GameEventResponse {
    pub id: i32,
    #[serde(rename = "type")]
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub payload: serde_json::Value,
}

#[derive(Serialize, Debug)]
pub struct PlayersResponse {
    pub players: Vec<PlayersResponsePlayer>,
}

#[derive(Serialize, Debug)]
pub struct PlayersResponsePlayer {
    pub username: String,
    pub agent: String,
}

#[derive(Serialize, Debug)]
pub struct GameResultResponse {
    pub result: Option<i32>,
}

#[derive(Serialize, Debug)]
pub struct GameResponse {
    #[serde(rename = "id")]
    pub game_id: i32,
    pub queued_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Debug)]
pub struct ActiveGamesResponse {
    pub games: Vec<GameResponse>,
}

#[derive(Serialize, Debug)]
pub struct ActiveAgentResponse {
    pub active_agent: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct UserScoreResponse {
    pub agent: String,
    pub score: Option<i32>,
}
