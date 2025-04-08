#![allow(unused)]

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, warn};

use crate::config::DiscordOauthSettings;
use core_lib::errors::{CoreError, Result as CoreResult};
use core_lib::models::user::{DiscordTokenResponse, DiscordUserProfile};

#[derive(Serialize)]
struct TokenExchangeRequest<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    grant_type: &'static str,
    code: &'a str,
    redirect_uri: &'a str,
}

#[derive(Deserialize, Debug)]
struct DiscordApiError {
    error: String,
    error_description: Option<String>,
    message: Option<String>, // ? A volte usano 'message' invece di 'error' by Gemini 2.5
    code: Option<i32>,
}

pub async fn exchange_code_for_token(
    code: &str,
    oauth_settings: &DiscordOauthSettings,
) -> CoreResult<DiscordTokenResponse> {
    debug!("Exchanging Discord code for token...");

    let client = Client::new();
    let token_endpoint = "https://discord.com/api/v10/oauth2/token";

    let request_body = TokenExchangeRequest {
        client_id: &oauth_settings.client_id,
        client_secret: &oauth_settings.client_secret,
        grant_type: "authorization_code",
        code,
        redirect_uri: &oauth_settings.redirect_uri,
    };

    let response = client
        .post(token_endpoint)
        .form(&request_body)
        .send()
        .await
        .map_err(|e| {
            error!("HTTP request to Discord token endpoint failed: {}", e);
            CoreError::ExternalService {
                service: "Discord".to_string(),
                message: format!("Network error during token exchange: {}", e),
            }
        })?;

    let status = response.status();
    let response_body_text = response.text().await.map_err(|e| {
        error!("Failed to read Discord token response body: {}", e);
        CoreError::ExternalService {
            service: "Discord".to_string(),
            message: format!("Error reading token response body: {}", e),
        }
    })?;

    debug!(
        "Discord token exchange response status: {}. Body: {:#?}",
        status,
        &response_body_text[..100.min(response_body_text.len())]
    );

    if status.is_success() {
        debug!(
            "Successfully exchanged code for token. Response body: {}",
            &response_body_text[..100.min(response_body_text.len())]
        );

        serde_json::from_str::<DiscordTokenResponse>(&response_body_text).map_err(|e| {
            error!(
                "Failed to deserialize Discord token response: {}. Body: {}",
                e, response_body_text
            );
            CoreError::ExternalService {
                service: "Discord".to_string(),
                message: format!("Failed to parse token response: {}", e),
            }
        })
    } else {
        warn!(
            "Discord token exchange failed with status: {}. Body: {}",
            status, response_body_text
        );
        // Prova a deserializzare l'errore da Discord
        let discord_error: DiscordApiError = serde_json::from_str(&response_body_text)
            .unwrap_or_else(|_| DiscordApiError {
                // Fallback se non deserializzabile
                error: format!("HTTP Error {}", status),
                error_description: Some(response_body_text),
                message: None,
                code: None,
            });

        Err(CoreError::ExternalService {
            service: "Discord".to_string(),
            message: format!(
                "Token exchange failed: {} - {:?}",
                discord_error.error,
                discord_error.error_description.or(discord_error.message)
            ),
        })
    }
}

// Ottieni il profilo dell'utente autenticato da Discord
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
            &response_body_text[..100.min(response_body_text.len())]
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
        let discord_error: DiscordApiError = serde_json::from_str(&response_body_text)
            .unwrap_or_else(|_| DiscordApiError {
                // Fallback se non deserializzabile
                error: format!("HTTP Error {}", status),
                error_description: Some(response_body_text),
                message: None,
                code: None,
            });

        Err(CoreError::ExternalService {
            service: "Discord".to_string(),
            message: format!(
                "User profile fetch failed: {} - {:?}",
                discord_error.error,
                discord_error.message // ? Discord spesso usa 'message' per errori API by Gemini 2.5
                .or(discord_error.error_description)
                .or_else(|| discord_error.code.map(|code| code.to_string()))
            ),
        })
    }
}
