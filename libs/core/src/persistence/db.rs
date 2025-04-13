// libs/core_lib/src/persistence/db.rs
use crate::config_models::DatabaseSettings; 
use crate::errors::{CoreError, Result};
use surrealdb::engine::any::{self, Any}; 
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tracing::info;

pub async fn create_surreal_connection(settings: &DatabaseSettings) -> Result<Surreal<Any>> {
    info!("Connecting to SurrealDB at {}", settings.url);

    let db= any::connect(&settings.url)
        .await
        .map_err(|e| CoreError::DatabaseConnect(e.to_string()))?; 

    if let (Some(ns), Some(db_name)) = (&settings.namespace, &settings.database_name) {
         info!("Using Namespace: {}, Database: {}", ns, db_name);
         db.use_ns(ns).use_db(db_name).await
            .map_err(|e| CoreError::DatabaseSetup(e.to_string()))?;
    } else {
        return Err(CoreError::DatabaseSetup("Namespace or Database name not provided".to_string()));
    }

    // @todo Aggiungi qui la logica di autenticazione se necessaria
    if let (Some(user), Some(pass)) = (&settings.username, &settings.password) {
        db.signin(Root {
            username: user,
            password: pass,
            // ... altri campi per l'auth scope se servono
        }).await.map_err(|e| CoreError::DatabaseAuth(e.to_string()))?;
        info!("Signed in to SurrealDB successfully");
    }

    Ok(db)
}