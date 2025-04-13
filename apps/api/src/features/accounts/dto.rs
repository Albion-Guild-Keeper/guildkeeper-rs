use serde::{Serialize, Deserialize};
use utoipa::ToSchema; 
use validator::Validate;
use core_lib::models::account::Account as AccountModel; 

#[derive(Serialize, ToSchema, Debug, Clone)]
#[schema(description = "Represents a account profile")]
pub struct AccountResponse {
    #[schema(example = "account:johndoe123")]
    pub id: String, 
    #[schema(example = "John Doe")]
    pub username: String,
    #[schema(example = "john.doe@example.com")]
    pub email: Option<String>,
    #[schema(example = "123456789012345678")] 
    pub discord_id: Option<String>,
    #[schema(example = "https://cdn.discordapp.com/.../avatar.png")]
    pub discord_avatar_url: Option<String>,
    #[schema(example = json!(["member", "moderator"]))]
    pub roles: Vec<String>,
}

// Implementa una conversione dal modello del DB al DTO di risposta
impl From<AccountModel> for AccountResponse {
    fn from(account_model: AccountModel) -> Self {
        // Costruisci l'URL dell'avatar se hai l'ID e l'hash
        let avatar_url = match (&account_model.discord_id, &account_model.discord_avatar) {
            (Some(id), Some(hash)) => Some(format!("https://cdn.discordapp.com/avatars/{}/{}.png", id, hash)),
            _ => None,
        };

        AccountResponse {
            id: account_model.id.to_string(), // Converti RecordId a String
            username: account_model.username,
            email: account_model.email,
            discord_id: account_model.discord_id.map(|id| id.to_string()),
            discord_avatar_url: avatar_url,
            roles: account_model.roles,
        }
    }
}


// --- DTO per la Creazione (POST /accounts) ---
// (Esempio, potrebbe non essere necessaria se crei solo via Discord OAuth)
#[derive(Deserialize, ToSchema, Debug, Validate)] // Aggiungi Validate se usi la crate
#[schema(description = "Payload for creating a new account (example, likely unused if only Discord OAuth)")]
pub struct CreateAccountRequest {
    #[schema(example = "Jane Doe")]
    #[validate(length(min = 3, message = "Accountname must be at least 3 characters"))]
    pub username: String,

    #[schema(example = "jane.doe@example.com")]
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>, // Rendi opzionale o obbligatorio a seconda delle regole

    #[schema(example = "a-secure-password")]
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: Option<String>, // SOLO se permetti login con password

    // Non includere discord_id qui, viene da OAuth
}

// --- DTO per l'Aggiornamento (PUT /accounts/{id} o PATCH) ---
// (Esempio, adatta ai campi che permetti di aggiornare)
#[derive(Deserialize, ToSchema, Debug, Validate)]
#[schema(description = "Payload for updating account details")]
pub struct UpdateAccountRequest {
    #[schema(example = "Johnny Doe")]
    #[validate(length(min = 3, message = "Accountname must be at least 3 characters"))]
    pub username: Option<String>, // Rendi i campi opzionali per PATCH

    #[schema(example = "johnny.doe@newdomain.com")]
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    // Aggiungi altri campi aggiornabili...
    // NON permettere di aggiornare discord_id, password (usa flusso separato), roles (usa endpoint separato)
}