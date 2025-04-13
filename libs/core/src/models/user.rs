// IN: libs/core_lib/src/models/user.rs
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use surrealdb::RecordId; // Per RecordId (es. "user:uuid()")
use surrealdb::sql; // Per poter usare sql::Thing o altri tipi specifici se necessario
use std::str::FromStr;

// La struct che rappresenta un utente nel nostro sistema e nel DB
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    #[serde(default = "User::default_id")]
    pub id: RecordId,
    pub username: String
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