use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GuildsList {
    pub guilds: Vec<Guild>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub owner: bool,
    pub permissions: u64,
    pub features: Vec<String>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub flags: u64,
    pub boost_count: u64,
}