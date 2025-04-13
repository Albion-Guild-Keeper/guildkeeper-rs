use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildsListResponse {
    pub guilds: Vec<GuildResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildResponse {
    pub name: String,
    pub icon: Option<String>,
}