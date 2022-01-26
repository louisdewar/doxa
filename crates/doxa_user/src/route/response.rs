use doxa_db::serde_json;
use serde::Serialize;

#[derive(Serialize)]
pub struct PublicUserInfo {
    pub username: String,
    pub admin: bool,
    pub competitions: Vec<String>,
    pub extra: serde_json::Value,
}
