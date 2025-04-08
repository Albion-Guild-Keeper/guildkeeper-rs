// IN: apps/rest_api/src/features/users/handler.rs
use actix_web::{get, post, web, HttpMessage, HttpResponse, Responder, Result as ActixResult};
use tracing::{error, info};
use crate::{errors::ApiError, middleware::auth::AuthenticatedUser, state::AppState};

use super::dto::UserResponse; 

// --- Handler per Creare Utente (ipotetico, non protetto) ---
#[utoipa::path(/* ... */)]
#[post("")] // POST /api/v1/users
pub async fn create_user_handler(// ... (body: web::Json<dto::CreateUserRequest>, state: web::Data<AppState>) ...
) -> ActixResult<impl Responder, ApiError> {
    // ... logica creazione ...
    Ok(HttpResponse::Created().finish()) // Esempio
}

// --- Handler per Ottenere Utente per ID (ipotetico, non protetto) ---
#[utoipa::path(/* ... */)]
#[get("/{user_id}")] // GET /api/v1/users/{user_id}
pub async fn get_user_handler(// ... (path: web::Path<String>, state: web::Data<AppState>) ...
) -> ActixResult<impl Responder, ApiError> {
    // ... logica fetch utente per ID ...
    Ok(HttpResponse::Ok().finish()) // Esempio
}

// ===>>> Handler di Test Protetto <<<===
/// Gets the profile details for the currently logged-in user.
/// Requires a valid Bearer token in the Authorization header.
#[utoipa::path(
    get,
    path = "/api/v1/users/@me", // Endpoint per l'utente corrente
    responses(
        (status = 200, description = "Current user profile", body = UserResponse), // Usa il tuo DTO UserResponse
        (status = 401, description = "Unauthorized - Invalid or missing token", body = crate::features::auth::dto::ErrorResponse) // Riferisciti al DTO di errore auth
    ),
    security( // Specifica che richiede l'autenticazione Bearer
        ("bearer_auth" = [])
    ),
    tags = ["Users"]
)]
#[get("/@me")] // GET /api/v1/users/@me
pub async fn get_current_user_handler(
    // NON ricevi più AuthenticatedUser direttamente come argomento con questo tipo di middleware
    // Devi estrarlo dalle estensioni della richiesta
    req: actix_web::HttpRequest, // Ottieni l'oggetto HttpRequest
    state: web::Data<AppState>,
) -> ActixResult<impl Responder, ApiError> {
    // Estrai l'utente autenticato inserito dal middleware
    // req.extensions() ritorna una mappa immutabile, .get() ritorna Option<&T>
    let authenticated_user = req.extensions().get::<AuthenticatedUser>().cloned(); // Clona se trovato

    if let Some(auth_user) = authenticated_user {
        info!(
            "Fetching profile for authenticated user ID: {}",
            auth_user.user_id
        );

        // Converti l'ID stringa in RecordId se necessario
        let user_id: surrealdb::sql::Thing =
            match core_lib::models::user::parse_id_from_string(&auth_user.user_id) {
                Ok(id) => id,
                Err(_) => {
                    return Err(ApiError::InternalServer(
                        "Invalid user ID format found in token".to_string(),
                    ))
                }
        };

        // Usa l'ID per recuperare i dettagli completi dell'utente dal DB
        // The find_by_id function expects a RecordId, but we have a Thing.
        // Convert the Thing to a RecordId before passing it to the function.
        let record_id: surrealdb::RecordId = user_id.to_string().parse().expect("Failed to parse RecordId");
        match core_lib::persistence::user_repo::find_by_id(&state.db, &record_id).await {
            Ok(Some(user)) => {
                // Converti in DTO e restituisci
                let response_dto = super::dto::UserResponse::from(user);
                Ok(HttpResponse::Ok().json(response_dto))
            }
            Ok(None) => {
                // Questo non dovrebbe succedere se il token è valido e l'utente esiste
                error!(
                    "User ID {} from valid token not found in DB!",
                    auth_user.user_id
                );
                Err(ApiError::InternalServer(
                    "Authenticated user not found".to_string(),
                ))
            }
            Err(core_err) => {
                error!("DB error fetching user {}: {}", auth_user.user_id, core_err);
                Err(ApiError::from(core_err))
            }
        }
    } else {
        // Questo non dovrebbe accadere se il middleware è configurato correttamente
        // perché il middleware dovrebbe restituire 401 prima di arrivare qui.
        // Ma è bene gestire il caso per robustezza.
        error!(
            "AuthenticatedUser not found in request extensions after Authentication middleware."
        );
        Err(ApiError::InternalServer(
            "Authentication context missing".to_string(),
        ))
    }
}

// --- Aggiungi una funzione helper per parsare l'ID in models/user.rs ---
/*
// IN: libs/core_lib/src/models/user.rs
use std::str::FromStr;
use surrealdb::sql::Thing; // Importa Thing

impl User {
    // ... altri metodi ...

    // Funzione per parsare l'ID stringa dal token nel formato Thing (table:id)
    pub fn parse_id_from_string(id_str: &str) -> std::result::Result<Thing, &'static str> {
        Thing::from_str(id_str).map_err(|_| "Invalid RecordId format")
    }
}
*/
