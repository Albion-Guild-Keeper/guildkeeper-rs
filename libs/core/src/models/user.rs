use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
use surrealdb::sql;
use surrealdb::RecordId;

// La struct che rappresenta un utente nel nostro sistema e nel DB
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    #[serde(default = "User::default_id")]
    pub id: RecordId,
    pub username: String,
    pub achivements_id: Vec<i64>,
    pub attendance: i64,
    pub created_at: String,
    pub updated_at: String,
    pub user_id: String,
    pub vod: i64,
    pub ign_link: String,
    pub masteries: Vec<String>,
    pub roles_id: Vec<i64>,
    pub balance: i64,
}

impl User {
    // Funzione helper per fornire un ID di default se non specificato
    // nel JSON/dati deserializzati. In questo caso, usiamo un ID basato su UUID.
    // Adatta la tabella ("user") al nome che usi in SurrealDB.
    pub fn default_id() -> RecordId {
        // Crea un ID nel formato "table:ulid" o "table:uuid"
        RecordId::from(("user", uuid::Uuid::new_v4().to_string()))
    }

    // Potresti aggiungere un costruttore qui se la logica di creazione è complessa
    // pub fn new_from_discord(profile: &DiscordUserProfile) -> Self { ... }
}

pub fn parse_id_from_string(id_str: &str) -> std::result::Result<sql::Thing, &'static str> {
    sql::Thing::from_str(id_str).map_err(|_| "Invalid ID format")
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindRelUserIdGuildId {
    pub users: Option<Vec<UserRel>>,
    pub guilds: Option<Vec<GuildRel>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildRel {
    pub id: RecordId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRel {
    pub id: String,
    pub username: String,
    // other fields...
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelAccountUser {
    #[serde(rename = "in")]
    pub account: RecordId,
    #[serde(rename = "out")]
    pub user: RecordId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelUserGuild {
    #[serde(rename = "in")]
    pub user: RecordId,
    #[serde(rename = "out")]
    pub guild: RecordId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGuilds {
    pub guilds: Vec<Guild>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildsList {
    pub guilds: Vec<Guild>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Guild {
    pub id: RecordId,
    pub name: String,
    pub guild_id: i64,
    pub balance: i64,
    pub applications_open: bool,
    pub created_at: String,
    pub updated_at: String,
}

// Serializza l'id come stringa
impl Serialize for Guild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("Guild", 7)?;
        s.serialize_field("id", &self.id.to_string())?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("guild_id", &self.guild_id)?;
        s.serialize_field("balance", &self.balance)?;
        s.serialize_field("applications_open", &self.applications_open)?;
        s.serialize_field("created_at", &self.created_at)?;
        s.serialize_field("updated_at", &self.updated_at)?;
        s.end()
    }
}

// The query returns a Vec of objects with a field `guild_ids` which is a Vec of SurrealDB RecordId objects.
// We'll flatten and extract only the inner id string.
#[derive(Deserialize)]
pub struct GuildIdWrap {
    pub guild_ids: Vec<surrealdb::sql::Id>,
}
