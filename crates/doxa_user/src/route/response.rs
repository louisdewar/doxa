use serde::Serialize;

#[derive(Serialize)]
pub struct UserInfo {
    pub username: String,
    pub admin: bool,
    pub competitions: Vec<String>,
}
