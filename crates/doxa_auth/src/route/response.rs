use serde::Serialize;

#[derive(Serialize)]
pub struct Login {
    pub auth_token: String,
}
