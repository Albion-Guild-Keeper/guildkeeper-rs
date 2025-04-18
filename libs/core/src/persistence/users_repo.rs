use crate::errors::{CoreError, Result};
use crate::models::user::{FindRelUserIdGuildId, Guild, GuildsList, User};
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use tracing::{debug, error, info, warn};
use tracing_subscriber::field::debug;

pub const TABLE: &str = "users";
pub const TABLE_REL: &str = "joined";
pub const TABLE_ACCOUNT: &str = "accounts";

pub async fn create(db: &Surreal<Any>, new_data: User) -> Result<User> {
    debug!(username = ?new_data.username, "Creating new user");
    let created_user: User = db
        .create(TABLE)
        .content(new_data)
        .await?
        .ok_or_else(|| CoreError::Internal("Failed to create user".to_string()))?;

    info!(user = ?created_user, "User created");

    Ok(created_user)
}

pub async fn find_by_discord_id(db: &Surreal<Any>, discord_id: &str) -> Result<Vec<User>> {
    debug!(discord_id = ?discord_id, "Finding user by Discord ID");

    let query = "SELECT * FROM type::table($table) WHERE user_id = type::int($user_id)";
    let mut result = db.query(query)
        .bind(("table", TABLE))
        .bind(("user_id", discord_id.to_string()))
        .await?;

    let users: Vec<User> = result.take(0)?;

    Ok(users)
}

pub async fn relate_account_user(db: &Surreal<Any>, user_id: &str, account_id: &str) -> Result<()> {
    debug!(user_id = ?user_id, account_id = ?account_id, "Relating user to account");

    let query = "
    INSERT RELATION INTO connected {
        in: type::record($account_id),
        out: type::record($user_id)
    }";

    db.query(query)
        .bind(("table", TABLE_REL))
        .bind(("user_id", user_id.to_string()))
        .bind(("account_id", account_id.to_string()))
        .await?;

    info!("User {} related to account {}", user_id, account_id);

    Ok(())
}

pub async fn get_guilds_by_account_id(db: &Surreal<Any>, account_id: &str) -> Result<Vec<GuildsList>> {
    debug!(account_id = ?account_id, "Finding guilds by account ID");

    let query = "
    SELECT ->connected->users->joined->guilds.* AS guilds
    FROM type::table($table)
    WHERE id = type::record($account_id);";

    let mut result = db.query(query)
        .bind(("table", TABLE_ACCOUNT))
        .bind(("account_id", account_id.to_string()))
        .await?;

    let guilds: Vec<GuildsList> = result.take(0)?;

    Ok(guilds)
}

// NOTE: This function now expects FindRelUserIdGuildId to contain Vec<RecordId> directly.
// Ensure the struct definition in models::user is updated accordingly.
pub async fn find_by_user_id_into_guild(db: &Surreal<Any>, user_id: &str, guild_id: &str) -> Result<FindRelUserIdGuildId> {
    warn!(user_id = ?user_id, guild_id = ?guild_id, "Finding user by ID into guild ID");

    // Modifica query SQL: Seleziona direttamente gli ID in array
    // Questo dovrebbe restituire un oggetto con campi 'users' e 'guilds',
    // ognuno contenente un array di RecordId (stringhe "table:id").
    let sql = "
    SELECT
        [ in.id ] AS users,    -- Seleziona l'ID dell'utente in un array
        [ out.id ] AS guilds   -- Seleziona l'ID della gilda in un array
    FROM type::table($table)
    WHERE in = type::thing('users', $user_id) AND out = type::thing('guilds', $guild_id)
    LIMIT 1;"; // Limita a 1 perché ci aspettiamo una sola relazione

    // Binding remains the same, ensure user_id and guild_id are just the ID part
    let mut response = db
        .query(sql)
        .bind(("table", TABLE_REL))
        .bind(("user_id", user_id.to_string())) // Clone user_id
        .bind(("guild_id", guild_id.to_string())) // Clone guild_id
        .await?;

    // Deserializza nella struct FindRelUserIdGuildId (che ora deve aspettarsi Vec<RecordId>)
    let results: Vec<FindRelUserIdGuildId> = response.take(0)?;

    debug!("Results from query: {:#?}", results);

    // La logica di controllo del risultato rimane valida
    if let Some(result) = results.into_iter().next() {
        // Controlla se gli array di ID non sono vuoti
        if result.users.as_ref().map_or(false, |v| !v.is_empty()) && result.guilds.as_ref().map_or(false, |v| !v.is_empty()) {
            info!("Relationship found: {:#?}", result);
            Ok(result)
        } else {
             warn!("Relationship found but user or guild ID array is missing or empty in the result object");
             Err(CoreError::NotFound("Relationship found but user or guild link is missing".to_string()))
        }
    } else {
        warn!("No relationship found between the given user ID and guild ID");
        Err(CoreError::NotFound("No relationship found".to_string()))
    }
}

// pub async fn get_user_guilds(db: &Surreal<Any>, user_id: &str, account_id: &str) -> Result<Vec<User>> {
//     debug!(user_id = ?user_id, account_id = ?account_id, "Getting user guilds");

//     let query = "
//     SELECT ->connected->users->joined->guilds.* AS guilds
//     FROM type::table($table)
//     WHERE id = $account_id";

//     let mut response = db
//         .query(query)
//         .bind(("table", TABLE_REL))
//         .bind(("account_id", account_id.to_string())) // Use account_id instead of user_id
//         .await?;

//     let results: Vec<FindRelUserIdGuildId> = response.take(0)?;

//     debug!("Results from query: {:#?}", results);

//     Ok()
// }