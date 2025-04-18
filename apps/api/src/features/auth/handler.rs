// IN: apps/rest_api/src/features/auth/handler.rs
use super::service;
use crate::state::AppState;
use crate::{errors::ApiError, features::auth::dto::LoginResponse};
use actix_session::Session;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder, Result as ActixResult};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD as base64_url, Engine as _};
use rand::Rng;
use tracing::{debug, error, info, warn};

const OAUTH_STATE_KEY: &str = "oauth_state"; // @todo spostare in un file di configurazione

#[utoipa::path(/* ... */)]
#[get("/login")] //@todo //! da cambiare in POST é GET solo per TEST
pub async fn discord_login_handler(
    query: web::Query<super::dto::LoginQuery>,
    state: web::Data<AppState>,
    session: Session,
) -> ActixResult<impl Responder, ApiError> {
    let mut rng = rand::rng();
    let csrf_bytes: [u8; 32] = rng.random();
    let csrf_state = base64_url.encode(csrf_bytes);
    let last_page = query.last_page.clone();

    debug!("Last page to redirect after login: {}", last_page);

    session.insert(OAUTH_STATE_KEY, &csrf_state).map_err(|e| {
        error!("Failed to insert CSRF state into session: {}", e);
        ApiError::InternalServer("Failed to prepare authentication flow".to_string())
    })?;

    debug!("Generated and saved CSRF state: {}", csrf_state);

    let redirect_url =
        service::generate_discord_auth_url(&state.settings.discord_oauth, &csrf_state).map_err(
            |e| {
                error!("Failed to generate Discord auth URL: {}", e);
                ApiError::InternalServer("Failed to build redirect URL".to_string())
            },
        )?;

    let cookie = actix_web::cookie::Cookie::build("last_page", last_page)
        .path("/") // Definisci il percorso in cui il cookie è valido
        // .secure(true) Imposta il flag Secure se necessario (HTTPS)
        .http_only(true) // Impedisce l'accesso al cookie tramite JavaScript
        .finish();

    // 4. Reindirizza l'utente
    Ok(HttpResponse::Found()
        .cookie(cookie) 
        .append_header(("Location", redirect_url))
        .finish())
}

#[utoipa::path(/* ... */)]
#[get("/callback")]
pub async fn discord_callback_handler(
    req: HttpRequest,
    query: web::Query<super::dto::CallbackQuery>,
    state: web::Data<AppState>,
    session: Session,
) -> ActixResult<impl Responder, ApiError> {
    let saved_state: Option<String> = session.get(OAUTH_STATE_KEY).map_err(|e| {
        error!("Failed to retrieve CSRF state from session: {}", e);
        ApiError::InternalServer("Failed to verify authentication flow".to_string())
    })?;

    debug!("Retrieved CSRF state from session: {:?}", saved_state);

    session.remove(OAUTH_STATE_KEY);

    // 2. Verifica lo stato CSRF
    match saved_state {
        Some(saved) if saved == query.state => {
            debug!("CSRF state verified successfully.");
            // Lo stato corrisponde, procedi...
        }
        _ => {
            warn!(
                "CSRF state mismatch or not found. Provided: '{}', Expected: '{:?}'",
                query.state, saved_state
            );
            // Explicitly state the Ok type (HttpResponse) for the Err variant
            return Err::<HttpResponse, _>(ApiError::BadRequest(
                "Invalid or missing state parameter".to_string(),
            ));
        }
    }

    // 2. Chiama il servizio per scambiare codice, ottenere utente, generare JWT locale
    //    Passa le parti necessarie dallo stato (db, settings)
    let handle_result = service::handle_discord_callback(
        &query.code,
        &state.db,       // Passa riferimento alla connessione DB
        &state.settings, // Passa riferimento alle Settings (che sono dentro Arc)
    )
    .await; // Il servizio ritorna CoreResult<String>

    // 3. Gestisci il risultato del servizio
    match handle_result {
        // --- Caso di Successo: Il servizio ha restituito il nostro JWT ---
        Ok(local_jwt) => {
            info!("Successfully handled Discord callback and generated local JWT.");
            // Costruisci il corpo della risposta JSON usando il DTO LoginResponse
            let _response_body = LoginResponse {
                access_token: local_jwt.clone(),
                token_type: "Bearer".to_string(),
            };

            // Crea un cookie per memorizzare il JWT
            let cookie =
                actix_web::cookie::Cookie::build(&state.settings.cookie_jwt_name, local_jwt)
                    .path("/") // Definisci il percorso in cui il cookie è valido
                    // .secure(true) // Imposta il flag Secure se necessario (HTTPS)
                    .http_only(true) // Impedisce l'accesso al cookie tramite JavaScript
                    .finish();

            let mut last_page: Option<String> = req.cookie("last_page").map(|c| c.value().to_string());

            if last_page.as_deref() == Some("Main") {
                last_page = Some("".to_string()); // Se l'ultima pagina è "main", reindirizza alla home page
            }

            req.cookie("last_page").take(); // Rimuovi il cookie last_page

            let redirect_url = format!("http://localhost:8080/{}", last_page.unwrap_or_else(|| "".to_string()).to_lowercase());

            debug!("Redirecting to: {}", redirect_url);

            // Inserisci il cookie nella risposta
            // Restituisci una risposta HTTP 200 OK con il corpo JSON
            Ok(HttpResponse::Found()
                .cookie(cookie) // Allega il cookie alla risposta di reindirizzamento
                .append_header(("Location", redirect_url)) // Imposta l'URL di reindirizzamento
                .finish())
        }


        // --- Caso di Errore: Il servizio ha restituito un CoreError ---
        Err(core_error) => {
            // Logga l'errore dettagliato che viene da core_lib
            error!("Error handling Discord callback in service: {}", core_error);

            // ===>>> Converti CoreError in ApiError <<<===
            // L'implementazione `From<CoreError> for ApiError` che abbiamo definito
            // in `errors.rs` si occupa di questa conversione.
            // Questa conversione mappa errori interni a codici/messaggi HTTP appropriati.
            let api_error = ApiError::from(core_error);

            // Restituisci l'errore ApiError. Actix userà l'implementazione
            // `ResponseError` di `ApiError` per generare la `HttpResponse`
            // con lo status code e il corpo JSON corretti.
            Err(api_error)
        }
    }
} // Fine di discord_callback_handler

#[utoipa::path(/* ... */)]
#[delete("/logout")]
pub async fn discord_logout_handler(
    session: Session,
    state: web::Data<AppState>,
) -> ActixResult<impl Responder, ApiError> {
    // Rimuovi la sessione dell'utente
    session.purge(); // Pulisce la sessione corrente
    debug!("User session purged successfully");

    // Cancella il cookie jwt_token
    let cookie = actix_web::cookie::Cookie::build(&state.settings.cookie_jwt_name, "")
        .path("/")
        .max_age(actix_web::cookie::time::Duration::seconds(0)) // Imposta Max-Age=0 per eliminarlo
        // .secure(true) // Imposta il flag Secure se necessario (HTTPS)
        .http_only(true)
        .finish();

    // Imposta il cookie con Max-Age=0 per eliminarlo
    Ok(HttpResponse::Ok().cookie(cookie).body("Logged out"))
}
