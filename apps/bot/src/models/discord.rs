use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Discord {
    pub id: i64,
    pub discord_name: String,
    pub joined_at: String
}