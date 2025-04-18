use crate::errors::{CoreError, Result};
use crate::models::account::Account;
use surrealdb::engine::any::Any;
use surrealdb::RecordId;
use surrealdb::Surreal;
use tracing::{debug, info, warn};

pub const TABLE: &str = "accounts"; 

// * Crea un nuovo utente
pub async fn create(db: &Surreal<Any>, new_account_data: Account) -> Result<Account> {
    debug!(accountname = %new_account_data.username, "Creating new account");
    let created_account: Account = db
        .create(TABLE)
        .content(new_account_data)
        .await?
        .ok_or_else(|| CoreError::Internal("Failed to create account".to_string()))?;

    Ok(created_account)
}

// * Aggiorna un utente esistente (sovrascrittura completa)
pub async fn update(db: &Surreal<Any>, account: &Account) -> Result<Account> {
    debug!(account_id = %account.id, "Updating account");
    let updated_account: Option<Account> = db
        .update((TABLE, account.id.clone().to_string()))
        .content(account.clone())
        .await?;

    debug!(account_id = %account.id, "Account updated successfully");

    updated_account.ok_or_else(|| {
        warn!(account_id = %account.id, "Account not found for update");
        CoreError::NotFound(format!("Account not found for update: {}", account.id))
    })
}

pub async fn delete(db: &Surreal<Any>, id: &RecordId) -> Result<Option<Account>> {
    debug!(account_id = %id, "Deleting account by RecordId");
    let deleted_account: Option<Account> = db.delete(id).await?;
     if deleted_account.is_some() {
        debug!(account_id = %id, "Account deleted successfully");
    } else {
        debug!(account_id = %id, "Account not found for deletion");
    }
    Ok(deleted_account)
}

// Trova un utente per discord_id (assumendo un campo 'discord_id' nella struct Account)
pub async fn find_by_discord_id(db: &Surreal<Any>, discord_id: &str) -> Result<Option<Account>> {
    debug!(discord_id = %discord_id, "Attempting to find account by discord_id");

    let query = "SELECT * FROM type::table($table) WHERE discord_id = type::int($discord_id) LIMIT 1;";
    let mut result = db.query(query)
        .bind(("table", TABLE))
        .bind(("discord_id", discord_id.to_string()))
        .await?; 

    let account: Option<Account> = result.take(0)?;

    if account.is_some() {
        debug!(discord_id = %discord_id, "Account found for discord_id");
    } else {
        debug!(discord_id = %discord_id, "No account found for discord_id");
    }

    Ok(account)
}

// Trova un utente per ID (RecordId)
pub async fn find_by_id(db: &Surreal<Any>, id: &RecordId) -> Result<Option<Account>> {
    debug!(account_id = %id, "Finding account by RecordId");
    let account: Option<Account> = db.select(id).await?;
     if account.is_some() {
        debug!(account_id = %id, "Account found");
    } else {
        debug!(account_id = %id, "Account not found");
    }
    Ok(account)
}

pub async fn relate_to_users(
    db: &Surreal<Any>,
    account_id: &RecordId,
    user_ids: &[RecordId],
) -> Result<()> {
    debug!(account_id = %account_id, "Relating account to users");

    let query = "UPDATE type::table($table) SET users = $user_ids WHERE id = $account_id;";
    
    Ok(())
}



