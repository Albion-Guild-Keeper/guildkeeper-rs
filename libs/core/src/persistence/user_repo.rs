// IN: libs/core_lib/src/persistence/user_repo.rs
use crate::errors::{CoreError, Result}; // Usa il Result definito in core_lib::errors
use crate::models::user::User;
use surrealdb::engine::any::Any;
use surrealdb::RecordId;
use surrealdb::Surreal;
use tracing::{debug, warn};

pub const USER_TABLE: &str = "user"; // Definisci il nome della tabella/risorsa

// Trova un utente per discord_id (assumendo un campo 'discord_id' nella struct User)
pub async fn find_by_discord_id(db: &Surreal<Any>, discord_id: &str) -> Result<Option<User>> {
    debug!(discord_id = %discord_id, "Attempting to find user by discord_id");

    // Usa una query SurrealQL per cercare sull'indice (se presente) o sul campo
    let query = "SELECT * FROM type::table($table) WHERE discord_id = $discord_id LIMIT 1;";
    let mut result = db.query(query)
        .bind(("table", USER_TABLE))
        .bind(("discord_id", discord_id.to_string()))
        .await?; // Propaga l'errore surrealdb::Error (convertito in CoreError::DatabaseQuery)

    // Prendi il primo (e unico) risultato se esiste
    // result.take(0) tenta di deserializzare la prima riga nella struct User
    let user: Option<User> = result.take(0)?; // Propaga errore di deserializzazione o DB

    if user.is_some() {
        debug!(discord_id = %discord_id, "User found for discord_id");
    } else {
        debug!(discord_id = %discord_id, "No user found for discord_id");
    }

    Ok(user)
}


// --- Assicurati che anche le altre funzioni create/update/delete siano qui ---

// Crea un nuovo utente
pub async fn create(db: &Surreal<Any>, new_user_data: User) -> Result<User> {
    debug!(username = %new_user_data.username, "Creating new user");
    // .create() restituisce Vec<User>, prendiamo il primo elemento.
    // L'ID in new_user_data viene usato se presente, altrimenti generato (a seconda del formato).
    let mut created_users: Vec<User> = db
        .create(USER_TABLE)
        .content(new_user_data) // Passa l'intera struct
        .await?
        .ok_or_else(|| CoreError::Internal("Failed to create user".to_string()))?;
    

    created_users.pop()
        .ok_or_else(|| {
            warn!("User creation attempt returned no record.");
            CoreError::Internal("Failed to create user or retrieve created record".to_string())
        })
}

// Aggiorna un utente esistente (sovrascrittura completa)
pub async fn update(db: &Surreal<Any>, user: &User) -> Result<User> {
    debug!(user_id = %user.id, "Updating user");
    // .update() sovrascrive l'intero record con i dati forniti.
    let updated_user: Option<User> = db
        .update((USER_TABLE, user.id.clone().to_string())) // Clona l'ID per passarlo
        .content(user.clone()) // Passa l'intera struct aggiornata
        .await?;


    debug!(user_id = %user.id, "User updated successfully");

    updated_user.ok_or_else(|| {
        warn!(user_id = %user.id, "User not found for update");
        CoreError::NotFound(format!("User not found for update: {}", user.id))
    })
}

// Trova un utente per ID (RecordId)
pub async fn find_by_id(db: &Surreal<Any>, id: &RecordId) -> Result<Option<User>> {
    debug!(user_id = %id, "Finding user by RecordId");
    let user: Option<User> = db.select(id).await?;
     if user.is_some() {
        debug!(user_id = %id, "User found");
    } else {
        debug!(user_id = %id, "User not found");
    }
    Ok(user)
}

// Cancella un utente per ID (RecordId)
pub async fn delete(db: &Surreal<Any>, id: &RecordId) -> Result<Option<User>> {
    debug!(user_id = %id, "Deleting user by RecordId");
    // delete ritorna il record cancellato se esisteva
    let deleted_user: Option<User> = db.delete(id).await?;
     if deleted_user.is_some() {
        debug!(user_id = %id, "User deleted successfully");
    } else {
        debug!(user_id = %id, "User not found for deletion");
    }
    Ok(deleted_user)
}