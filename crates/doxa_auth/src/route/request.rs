use serde::Deserialize;

#[derive(Deserialize)]
pub struct Register {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}
