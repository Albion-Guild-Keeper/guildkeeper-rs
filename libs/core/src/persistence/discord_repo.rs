use crate::errors::{CoreError, Result as CoreResult};
use crate::models::account::{Account, DiscordUserProfile};
use reqwest::Client;
use surrealdb::engine::any::Any;
use surrealdb::RecordId;
use surrealdb::Surreal;
use tracing::{debug, error, info, warn};

// Define DiscordErrorResponse struct
#[derive(Debug, serde::Deserialize, Default)]
struct DiscordErrorResponse {
    pub error: Option<String>,
    pub message: Option<String>,
    pub error_description: Option<String>,
    pub code: Option<u32>,
}

pub async fn get_discord_user_profile(access_token: &str) -> CoreResult<DiscordUserProfile> {
    debug!("Fetching Discord user profile (/users/@me)");

    let client = Client::new();
    let user_endpoint = "https://discord.com/api/v10/users/@me";

    let response = client
        .get(user_endpoint)
        .bearer_auth(access_token) // Usa il token nell'header Authorization: Bearer
        .send()
        .await
        .map_err(|e| {
            error!("HTTP request to Discord user endpoint failed: {}", e);
            CoreError::ExternalService {
                service: "Discord".to_string(),
                message: format!("Network error during profile fetch: {}", e),
            }
        })?;

    let status = response.status();
    let response_body_text = response.text().await.map_err(|e| {
        error!("Failed to read Discord user response body: {}", e);
        CoreError::ExternalService {
            service: "Discord".to_string(),
            message: format!("Error reading user response body: {}", e),
        }
    })?;

    if status.is_success() {
        debug!(
            "Successfully fetched user profile. Body: {}",
            &response_body_text
        );
        // Deserializza il profilo utente
        serde_json::from_str::<DiscordUserProfile>(&response_body_text).map_err(|e| {
            error!(
                "Failed to deserialize Discord user profile: {}. Body: {}",
                e, response_body_text
            );
            CoreError::ExternalService {
                service: "Discord".to_string(),
                message: format!("Failed to parse user profile response: {}", e),
            }
        })
    } else {
        warn!(
            "Discord user profile fetch failed with status: {}. Body: {}",
            status, response_body_text
        );
        let discord_error = serde_json::from_str::<DiscordErrorResponse>(&response_body_text).unwrap_or_default();
        error!(
            "Discord user profile fetch failed: {} - {:?}",
            format!("{:?}", discord_error.error),
            discord_error.message.clone() // ? Discord spesso usa 'message' per errori API by Gemini 2.5
            .or(discord_error.error_description.clone())
            .or_else(|| discord_error.code.map(|code| code.to_string()))
        );

        Err(CoreError::ExternalService {
            service: "Discord".to_string(),
            message: format!(
                "User profile fetch failed: {} - {:?}",
                format!("{:?}", discord_error.error),
                discord_error.message // ? Discord spesso usa 'message' per errori API by Gemini 2.5
                .or(discord_error.error_description)
                .or_else(|| discord_error.code.map(|code| code.to_string()))
            ),
        })
    }
}