// IN: apps/rest_api/src/errors.rs
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use std::fmt;
use utoipa::ToSchema; // Per la documentazione Swagger

// Importa l'errore condiviso
use core_lib::errors::CoreError;

// (Opzionale) Struct per il corpo JSON degli errori HTTP
#[derive(Serialize, ToSchema)]
pub struct ApiErrorResponse {
    #[schema(example = "ResourceNotFound")] // Codice errore breve
    error_code: String,
    #[schema(example = "User with ID 123 not found")]
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>, // Dettagli aggiuntivi (es. errori validazione)
}

// Enum per gli errori specifici dell'API
#[derive(Debug)] // Non deriva thiserror qui, lo gestiamo manualmente per ResponseError
pub enum ApiError {
    // Errori che originano da CoreError
    NotFound { resource: String, id: String },
    Database(CoreError), // Avvolge l'errore DB da core_lib
    Validation(Vec<String>),
    InternalServer(String), // Messaggio generico per errori non specificati
    ExternalService { service: String, message: String },
    Configuration(String),

    // Errori specifici SOLO dell'API
    BadRequest(String), // Es. parametri mancanti, formato JSON invalido
    Unauthorized(String), // Es. token JWT mancante o invalido, credenziali errate
    Forbidden(String), // Es. utente non ha i permessi
    Conflict(String), // Es. risorsa già esistente
    // Aggiungi altri errori specifici dell'API se necessario
}

// Implementazione per visualizzare l'errore (utile per i log)
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound { resource, id } => write!(f, "Resource '{}' with ID '{}' not found", resource, id),
            ApiError::Database(err) => write!(f, "Database Error: {}", err),
            ApiError::Validation(errs) => write!(f, "Validation Error: {:?}", errs),
            ApiError::InternalServer(msg) => write!(f, "Internal Server Error: {}", msg),
            ApiError::ExternalService { service, message } => write!(f, "External Service Error ({}): {}", service, message),
            ApiError::Configuration(msg) => write!(f, "Configuration Error: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            ApiError::Conflict(msg) => write!(f, "Conflict: {}", msg),
        }
    }
}

// ===>>> Implementazione Chiave: ResponseError per Actix <<<===
// Questo dice ad Actix come trasformare ApiError in una HttpResponse
impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::Validation(_) | ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::Database(_) |
            ApiError::InternalServer(_) |
            ApiError::ExternalService { .. } |
            ApiError::Configuration(_) => StatusCode::INTERNAL_SERVER_ERROR, // Mappa errori interni a 500
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let error_response = ApiErrorResponse {
            error_code: self.error_code_string(), // Funzione helper per codice breve
            message: self.to_string(), // Messaggio dettagliato da Display
            details: self.get_details(), // Funzione helper per dettagli extra
        };

        HttpResponse::build(status).json(error_response)
    }
}

// Funzioni helper private per ResponseError
impl ApiError {
    // Restituisce un codice stringa breve per l'errore
    fn error_code_string(&self) -> String {
        match self {
            ApiError::NotFound { .. } => "ResourceNotFound".into(),
            ApiError::Database(_) => "DatabaseError".into(),
            ApiError::Validation(_) => "ValidationError".into(),
            ApiError::InternalServer(_) => "InternalServerError".into(),
            ApiError::ExternalService { .. } => "ExternalServiceError".into(),
            ApiError::Configuration(_) => "ConfigurationError".into(),
            ApiError::BadRequest(_) => "BadRequest".into(),
            ApiError::Unauthorized(_) => "Unauthorized".into(),
            ApiError::Forbidden(_) => "Forbidden".into(),
            ApiError::Conflict(_) => "Conflict".into(),
        }
    }

    // Restituisce dettagli JSON opzionali (es. per validazione)
    fn get_details(&self) -> Option<serde_json::Value> {
        match self {
            ApiError::Validation(errors) => serde_json::to_value(errors).ok(),
            _ => None,
        }
    }
}


// ===>>> Implementazione From per convertire CoreError in ApiError <<<===
// Questo permette di usare l'operatore '?' sugli errori CoreError negli handler/service
impl From<CoreError> for ApiError {
    fn from(error: CoreError) -> Self {
        match error {
            CoreError::NotFound(msg) => {
                // Potresti voler estrarre resource/id dal msg se possibile,
                // altrimenti usa un messaggio generico
                ApiError::NotFound { resource: "Resource".to_string(), id: msg }
            }
            CoreError::Validation(errs) => ApiError::Validation(errs),
            CoreError::DatabaseConnect(_) |
            CoreError::DatabaseSetup(_) |
            CoreError::DatabaseAuth(_) |
            CoreError::DatabaseQuery(_) => ApiError::Database(error), // Avvolgi l'errore DB originale
            CoreError::ExternalService { service, message } => ApiError::ExternalService { service, message },
            CoreError::Configuration(msg) => ApiError::Configuration(msg),
            CoreError::Serialization(err) => ApiError::InternalServer(format!("Serialization error: {}", err)),
            // Mappa CoreError::Internal a ApiError::InternalServer
            CoreError::Internal(msg) => ApiError::InternalServer(msg),
        }
    }
}

// (Opzionale) Implementa From per altri errori comuni che possono verificarsi nell'API
// Esempio: Errore di deserializzazione JSON da Actix
impl From<actix_web::error::JsonPayloadError> for ApiError {
    fn from(error: actix_web::error::JsonPayloadError) -> Self {
        ApiError::BadRequest(format!("Invalid JSON payload: {}", error))
    }
}

// Esempio: Errore di parsing parametri Path
impl From<actix_web::error::PathError> for ApiError {
     fn from(error: actix_web::error::PathError) -> Self {
         ApiError::BadRequest(format!("Invalid path parameter: {}", error))
     }
}

// Aggiungi altre conversioni `From` necessarie (es. per errori JWT, ecc.)

// Alias Result specifico per l'API (utile negli handler)
pub type ApiResult<T> = std::result::Result<T, ApiError>;