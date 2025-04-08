// IN: libs/core_lib/src/errors.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    // Errore generico se non si sa cosa è andato storto
    #[error("Internal Server Error: {0}")]
    Internal(String),

    // Errore specifico per la connessione al DB
    #[error("Database connection failed: {0}")]
    DatabaseConnect(String),

    // Errore specifico per setup NS/DB (se distinto da query)
    #[error("Database namespace/db setup failed: {0}")]
    DatabaseSetup(String),

    // Errore specifico per l'autenticazione al DB (se distinta da query)
    #[error("Database authentication failed: {0}")]
    DatabaseAuth(String),

    // Errore dalle query SurrealDB (cattura errori DB generici)
    #[error("Database query failed: {0}")]
    DatabaseQuery(#[from] surrealdb::Error),

    // Errore se una risorsa attesa non viene trovata
    #[error("Resource not found: {0}")]
    NotFound(String), // Es: "User with ID xyz not found"

    // Errore per dati non validi (potrebbe venire da validazioni)
    #[error("Validation Error: {0:?}")]
    Validation(Vec<String>), // Vettore di messaggi di errore

    // Errore durante la (de)serializzazione
    #[error("Serialization/Deserialization Error: {0}")]
    Serialization(#[from] serde_json::Error), // Esempio con serde_json

    // Errore di configurazione (es. manca un valore richiesto)
    #[error("Configuration Error: {0}")]
    Configuration(String),

    // Errore durante chiamate HTTP a servizi esterni (es. Discord API)
    #[error("External Service Error ({service}): {message}")]
    ExternalService { service: String, message: String },
}

// Alias Result condiviso
pub type Result<T, E = CoreError> = std::result::Result<T, E>;