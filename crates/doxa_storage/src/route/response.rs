use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Upload {
    pub id: String,
    pub competition: String,
}
