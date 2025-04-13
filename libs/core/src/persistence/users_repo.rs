use crate::errors::{CoreError, Result};
use crate::models::user::{RelAccountUser, User};
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use tracing::{debug, info};

pub const TABLE: &str = "users";

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