use crate::errors::{CoreError, Result}; // Usa il Result definito in core_lib::errors
use crate::models::guilds::{GuildsList, Guild};
use surrealdb::engine::any::Any;
use surrealdb::RecordId;
use surrealdb::Surreal;
use tracing::{debug, warn};

pub const TABLE: &str = "guilds"; // Definisci il nome della tabella/risorsa

pub async fn find_by_account_id(db: &Surreal<Any>, account_id: &str) -> Result<Vec<Guild>> {
    debug!(account_id = %account_id, "Attempting to find guilds by account_id");

    let query = r#"
        SELECT guilds.*
        FROM type::table($table) AS guilds
        LET user = (SELECT * FROM users WHERE <-connected<-accounts WHERE discord_id = $account_id)
        WHERE guilds.id IN (SELECT ->joined->guilds.id FROM user)
    "#;
    let mut result = db.query(query)
        .bind(("table", TABLE))
        .bind(("account_id", account_id.to_string()))
        .await?; 
    let guilds: Vec<Guild> = result.take(0)?;
    Ok(guilds)
}