// IN: libs/core_lib/src/models/user.rs
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use surrealdb::RecordId; // Per RecordId (es. "user:uuid()")
use surrealdb::sql; // Per poter usare sql::Thing o altri tipi specifici se necessario
use std::str::FromStr;

// La struct che rappresenta un utente nel nostro sistema e nel DB
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    // ID del record nel formato SurrealDB (es. "user:uuid()" o "user:blaze")
    // Option<> se vogliamo che SurrealDB lo generi al .create() se non fornito
    // Ma è più semplice generarlo noi o usare un ID deterministico se possibile
    #[serde(default = "User::default_id")] // Se SurrealDB non lo genera automaticamente
    pub id: RecordId,

    // Nome utente (potrebbe venire da Discord o essere unico nel nostro sistema)
    pub username: String,

    // Email (opzionale, potrebbe essere richiesta con scope 'email')
    pub email: Option<String>,

    // ID Discord dell'utente (chiave per il lookup)
    // Rendi indicizzato nel DB se fai spesso ricerche su questo campo
    #[serde(default)] // Non fallire la deserializzazione se manca
    pub discord_id: Option<String>,

    // URL dell'avatar Discord (opzionale)
    #[serde(default)]
    pub discord_avatar: Option<String>,

    // Ruoli nel nostro sistema (esempio)
    #[serde(default)]
    pub roles: Vec<String>,

    // Timestamp di creazione e aggiornamento (opzionale)
    #[serde(default = "Utc::now")] // Usa Utc::now() come default alla deserializzazione
    pub created_at: DateTime<Utc>,

    #[serde(default, skip_serializing_if = "Option::is_none")] // Use Option::default() -> None
    pub updated_at: Option<DateTime<Utc>>,
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

// Struct per rappresentare i dati che riceviamo da Discord API /users/@me
// Definisci solo i campi che ti interessano.
#[derive(Debug, Deserialize)]
pub struct DiscordUserProfile {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>, // L'avatar è un hash, l'URL va costruito
    pub email: Option<String>, // Richiede lo scope 'email'
    // Aggiungi altri campi se necessari (discriminator, locale, etc.)
}

// Struct per rappresentare la risposta dal token endpoint di Discord
#[derive(Debug, Deserialize)]
pub struct DiscordTokenResponse {
    pub access_token: String,
    pub token_type: String, // Solitamente "Bearer"
    pub expires_in: u64, // Durata in secondi
    pub refresh_token: String,
    pub scope: String, // Scope concessi
}

pub fn parse_id_from_string(id_str: &str) -> std::result::Result<sql::Thing, &'static str> {
    // Assumendo che l'ID sia nel formato "user:uuid" o simile
    // Usa sql::Thing per rappresentare l'ID in SurrealDB
    sql::Thing::from_str(id_str).map_err(|_| "Invalid ID format")
}