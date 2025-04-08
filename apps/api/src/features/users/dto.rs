// IN: apps/rest_api/src/features/users/dto.rs
use serde::{Serialize, Deserialize};
use utoipa::ToSchema; // Per la documentazione Swagger
use validator::Validate; // Se usi la crate 'validator' per validazione
use core_lib::models::user::User as UserModel; // Importa il modello DB per conversioni

// --- DTO per la Risposta (GET /users/{id}, GET /users/@me) ---
#[derive(Serialize, ToSchema, Debug, Clone)]
#[schema(description = "Represents a user profile")]
pub struct UserResponse {
    #[schema(example = "user:johndoe123")] // Esempio ID SurrealDB
    pub id: String, // Restituisci l'ID come stringa nell'API
    #[schema(example = "John Doe")]
    pub username: String,
    #[schema(example = "john.doe@example.com")]
    pub email: Option<String>,
    #[schema(example = "123456789012345678")] // ID Discord
    pub discord_id: Option<String>,
    #[schema(example = "https://cdn.discordapp.com/.../avatar.png")]
    pub discord_avatar_url: Option<String>, // Potresti costruire l'URL qui
    #[schema(example = json!(["member", "moderator"]))]
    pub roles: Vec<String>,
    // Non includere campi sensibili come hash password!
    // Potresti aggiungere timestamp se rilevanti per il client
    // pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    // pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

// Implementa una conversione dal modello del DB al DTO di risposta
impl From<UserModel> for UserResponse {
    fn from(user_model: UserModel) -> Self {
        // Costruisci l'URL dell'avatar se hai l'ID e l'hash
        let avatar_url = match (&user_model.discord_id, &user_model.discord_avatar) {
            (Some(id), Some(hash)) => Some(format!("https://cdn.discordapp.com/avatars/{}/{}.png", id, hash)),
            _ => None,
        };

        UserResponse {
            id: user_model.id.to_string(), // Converti RecordId a String
            username: user_model.username,
            email: user_model.email,
            discord_id: user_model.discord_id,
            discord_avatar_url: avatar_url,
            roles: user_model.roles,
            // created_at: user_model.created_at,
            // updated_at: user_model.updated_at,
        }
    }
}


// --- DTO per la Creazione (POST /users) ---
// (Esempio, potrebbe non essere necessaria se crei solo via Discord OAuth)
#[derive(Deserialize, ToSchema, Debug, Validate)] // Aggiungi Validate se usi la crate
#[schema(description = "Payload for creating a new user (example, likely unused if only Discord OAuth)")]
pub struct CreateUserRequest {
    #[schema(example = "Jane Doe")]
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,

    #[schema(example = "jane.doe@example.com")]
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>, // Rendi opzionale o obbligatorio a seconda delle regole

    #[schema(example = "a-secure-password")]
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: Option<String>, // SOLO se permetti login con password

    // Non includere discord_id qui, viene da OAuth
}

// --- DTO per l'Aggiornamento (PUT /users/{id} o PATCH) ---
// (Esempio, adatta ai campi che permetti di aggiornare)
#[derive(Deserialize, ToSchema, Debug, Validate)]
#[schema(description = "Payload for updating user details")]
pub struct UpdateUserRequest {
    #[schema(example = "Johnny Doe")]
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: Option<String>, // Rendi i campi opzionali per PATCH

    #[schema(example = "johnny.doe@newdomain.com")]
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    // Aggiungi altri campi aggiornabili...
    // NON permettere di aggiornare discord_id, password (usa flusso separato), roles (usa endpoint separato)
}