// IN: apps/rest_api/src/features/auth/service.rs
use crate::{config::{DiscordOauthSettings, Settings}, features::auth::{client, dto::Claims}};
use chrono::{Duration, Utc};
use core_lib::{models::account::{self, Account}, persistence::{account_repo, discord_repo::get_discord_user_profile}, CoreError};
use jsonwebtoken::{encode, EncodingKey, Header};
use surrealdb::{engine::any::Any, Surreal};
use tracing::{debug, error, info, warn};
use url::Url; 

use core_lib::errors::Result as CoreResult;

pub fn generate_discord_auth_url(
    oauth_settings: &DiscordOauthSettings,
    csrf_state: &str, // <<< SECONDO ARGOMENTO: lo stato CSRF
) -> Result<String, url::ParseError> { // Restituisce Result per gestire errori URL

    let mut auth_url = Url::parse("https://discord.com/api/oauth2/authorize")?;

    auth_url.query_pairs_mut()
        .append_pair("client_id", &oauth_settings.client_id)
        .append_pair("redirect_uri", &oauth_settings.redirect_uri)
        .append_pair("response_type", "code")
        // Aggiungi gli scope necessari separati da spazio (codificati correttamente da `url`)
        .append_pair("scope", "identify email guilds") // Esempio scope
        .append_pair("state", csrf_state); // <<< USA LO STATE QUI

    // (Opzionale) Aggiungi prompt=none se vuoi tentare un login silenzioso
    // .append_pair("prompt", "none");

    Ok(auth_url.to_string())
}

// ... resto del service.rs (handle_discord_callback, etc.) ...
pub async fn handle_discord_callback(
    code: &str,
    db: &Surreal<Any>, // Passa la connessione DB direttamente
    settings: &Settings, // Passa le settings direttamente
    // Alternativa: state: &AppState // Passa l'intero stato se preferisci
) -> CoreResult<String> { // Restituisce Result<TUO_JWT, CoreError>

    info!("Handling Discord OAuth callback with code: {}", &code[0..5.min(code.len())]); // Logga solo l'inizio del codice

    // --- 1. Scambia il codice per il token di accesso Discord ---
    debug!("Exchanging authorization code for Discord token...");
    let discord_token_response = client::exchange_code_for_token(
        code,
        &settings.discord_oauth // Passa solo le impostazioni OAuth necessarie
    ).await?; // Propaga CoreError se fallisce
    debug!("Discord token obtained successfully.");

    // --- 2. Ottieni il profilo utente da Discord ---
    debug!("Fetching Discord user profile...");
    let discord_user_profile = get_discord_user_profile(
        &discord_token_response.access_token
    ).await?; // Propaga CoreError se fallisce
    info!("Fetched Discord profile for User ID: {}", discord_user_profile.id);


    // --- 3. Trova o Crea Utente nel TUO Database ---
    // Cerca l'utente tramite l'ID Discord
    let existing_account = account_repo::find_by_discord_id(
        db,
        &discord_user_profile.id
    ).await?;

    debug!("Searching for existing account in local DB...");

    let account_to_auth: Account = match existing_account {
        Some(mut account) => {
            info!("Found existing user in local DB with ID: {}", account.id);
            // (Opzionale) Aggiorna info come username/avatar se cambiate su Discord
            let mut needs_update = false;
            if account.username != discord_user_profile.username {
                account.username = discord_user_profile.username.clone();
                needs_update = true;
            }
            // Aggiungi controllo avatar, ecc.
            if needs_update {
                 debug!("Updating account info from Discord profile...");
                 match account_repo::update(db, &account).await {
                     Ok(updated) => account = updated, // Usa l'utente aggiornato
                     Err(e) => warn!("Failed to account user info for {}: {}", account.id, e), // Logga ma continua
                 }
            }
            account // Usa l'utente esistente (eventualmente aggiornato)
        }
        None => {
            info!("Account not found in local DB, creating new user for Discord ID: {}", discord_user_profile.id);
            let new_account_data = Account {
                id: Account::default_id(),
                username: discord_user_profile.username.clone(),
                email: discord_user_profile.email.clone(),
                discord_id: Some(discord_user_profile.id.parse::<i64>().unwrap()),
                discord_avatar: discord_user_profile.avatar,
                locale: discord_user_profile.locale.clone(),
                roles: vec!["default_role".to_string()], // @todo Aggiungi ruoli predefiniti con ENUM e poi verifiche varie
            };

            match account_repo::create(db, new_account_data).await {
                Ok(account) => account,
                Err(e) => {
                    error!("Failed to create account: {}", e);
                    return Err(e);
                }
            }
        }
    };

    info!("Account authenticated/created successfully. Local Account ID: {}", account_to_auth.id);


    // --- 4. Genera il TUO Token JWT ---
    debug!("Generating local JWT for user ID: {}", account_to_auth.id);
    let now = Utc::now();
    // Imposta la scadenza (es. 1 ora) - prendi la durata dalla configurazione!
    let expiration_time = now + Duration::hours(settings.jwt_expiration_hours.unwrap_or(1));

    let claims = Claims {
        sub: account_to_auth.id.to_string(), // Usa l'ID del TUO utente come subject
        exp: expiration_time.timestamp() as usize,
    };

    // Usa la chiave segreta dalla configurazione
    // IMPORTANTE: jwt_secret DEVE essere caricata in modo sicuro (da env/secrets)
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(settings.jwt_secret.as_ref()) // Usa il segreto dalla config
    ).map_err(|e| {
        error!("Failed to encode JWT: {}", e);
        CoreError::Internal("Failed to generate authentication token".to_string())
    })?;

    info!("JWT generated successfully for user ID: {}", account_to_auth.id);

    // --- 5. Restituisci il TUO token JWT ---
    Ok(token)
}