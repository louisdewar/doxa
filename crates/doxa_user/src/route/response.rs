use doxa_db::{model::user::User, serde_json};
use serde::Serialize;

#[derive(Serialize)]
pub struct PublicUserInfo {
    pub username: String,
    pub admin: bool,
    pub competitions: Vec<String>,
    pub extra: serde_json::Value,
}

#[derive(Serialize)]
/// Basic user info that can be given publicly with information that can be extracted from
/// [`doxa_db::model::user::User`].
pub struct PublicBasicUserInfo {
    pub username: String,
    pub admin: bool,
    pub extra: serde_json::Value,
}

impl From<User> for PublicBasicUserInfo {
    fn from(user: User) -> Self {
        PublicBasicUserInfo {
            username: user.username,
            admin: user.admin,
            extra: user.extra,
        }
    }
}
