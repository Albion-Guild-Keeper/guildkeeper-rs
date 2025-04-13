use serde::{Serialize, Deserialize};
use surrealdb::RecordId;
use surrealdb::sql;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Account {
    #[serde(default = "Account::default_id")]
    pub id: RecordId,
    pub username: String,
    pub email: Option<String>,
    pub discord_id: Option<i64>,
    #[serde(default)] 
    pub discord_avatar: Option<String>,
    #[serde(default)] // @todo Ruoli nel Sistema 
    pub roles: Vec<String>,
    #[serde(default)] 
    pub locale: Option<String>,
}

impl Account {
    pub fn default_id() -> RecordId {
        RecordId::from(("account", uuid::Uuid::new_v4().to_string()))
    }
}

// Struct per rappresentare i dati che riceviamo da Discord API /users/@me
// Definisci solo i campi che ti interessano.
#[derive(Debug, Deserialize)]
pub struct DiscordUserProfile {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>, // L'avatar è un hash, l'URL va costruito
    pub email: Option<String>, // Richiede lo scope 'email'
    pub global_name: Option<String>, // Richiede lo scope 'identify'
    pub locale: Option<String>, // Locale dell'utente (es. "it-IT")
    // Aggiungi altri campi se necessari (discriminator, locale, etc.)
}

// Struct per rappresentare la risposta dal token endpoint di Discord
#[derive(Debug, Deserialize)]
pub struct DiscordTokenResponse {
    pub access_token: String,
    pub token_type: String, 
    pub expires_in: u64, 
    pub refresh_token: String,
    pub scope: String, 
}

// @note Da controllare se é utile pare_id_from_string
pub fn parse_id_from_string(id_str: &str) -> std::result::Result<sql::Thing, &'static str> {
    sql::Thing::from_str(id_str).map_err(|_| "Invalid ID format")
}