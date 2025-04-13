// IN: apps/rest_api/src/middleware/auth.rs
use actix_web::web;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue, AUTHORIZATION},
    Error as ActixError,
    HttpMessage, // Importa HttpMessage per accedere alle estensioni
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::future::{ready, Ready};
use std::sync::Arc;
use tracing::{debug, error, warn};
use tracing_subscriber::field::debug;

// Importa lo stato e le settings per accedere alla chiave segreta JWT
use crate::errors::ApiError;
use crate::features::auth::dto::Claims;
use crate::state::AppState;

// --- Middleware Struct (Factory) ---
// Questa struct non contiene dati, serve solo come factory per il middleware effettivo.
#[derive(Clone)] // Necessario se passi lo stato qui, ma lo prendiamo dalle extensions
pub struct Authentication;

// Implementa Transform per creare l'istanza del middleware per ogni worker
impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static, // Aggiungi 'static bound
    S::Future: 'static, // Aggiungi 'static bound
    B: 'static,         // Aggiungi 'static bound
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type InitError = (); 
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Arc::new(service),
        })) // Avvolgi il servizio in Arc
    }
}

// --- Middleware Effettivo ---
// Questa struct contiene il servizio successivo nella catena
pub struct AuthenticationMiddleware<S> {
    // Usa Arc per condividere il servizio tra cloni del middleware
    // richiesto perché LocalBoxFuture lo richiede
    service: Arc<S>,
}

// Struct per contenere l'ID utente estratto dal token (opzionale ma utile)
// Lo aggiungeremo alle estensioni della richiesta per gli handler successivi
#[derive(Clone)]
pub struct AuthenticatedAccount {
    pub account_id: String, // O uuid::Uuid se usi UUID come ID
}

// Implementa Service per il middleware
impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static, // Aggiungi 'static bound
    S::Future: 'static, // Aggiungi 'static bound
    B: 'static,         // Aggiungi 'static bound
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    // Usiamo LocalBoxFuture perché la logica di validazione è async
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Pronti a processare la richiesta (di solito sempre pronto)
    forward_ready!(service);

    // Logica principale del middleware
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Clona il servizio Arc per passarlo al future
        let service = self.service.clone();

        Box::pin(async move {
            // 1. Estrai lo stato AppState (per accedere a jwt_secret)
            //    Usiamo le extensions della richiesta perché è il modo standard
            //    per accedere allo stato dentro un middleware di tipo Transform.
            //    Lo stato viene aggiunto con .app_data() in main.rs.
            let state = req.app_data::<web::Data<AppState>>().cloned(); // Clona web::Data<AppState>
            if state.is_none() {
                error!("AppState not found in request extensions. Middleware misconfigured?");
                // Restituisci un errore interno generico qui, perché è un problema di setup
                // Converti l'errore ApiError in ActixError
                let api_err =
                    ApiError::InternalServer("Middleware configuration error".to_string());
                return Err(ActixError::from(api_err));
            }
            let state = state.unwrap(); // Ora abbiamo web::Data<AppState>
            let jwt_secret = state.settings.jwt_secret.clone(); // Clona il segreto JWT
            warn!(
                "JWT secret loaded from settings: {}",
                jwt_secret // Logga il segreto per debug (NON in produzione!)
            );

            // 2. Estrai l'header Authorization
            let auth_header = req.headers().get(AUTHORIZATION);

            let token = match auth_header {
                Some(header_value) => {
                    // Prova a estrarre il token "Bearer <token>"
                    if let Ok(header_str) = header_value.to_str() {
                        if header_str.starts_with("Bearer ") {
                            debug!(
                                "Authorization header found: {}",
                                header_str // Logga l'header completo per debug
                            );
                            Some(header_str[7..].to_string()) // Estrai solo il token
                        } else {
                            None // Formato header non corretto
                        }
                    } else {
                        None // Header non è stringa UTF-8 valida
                    }
                }
                None => {
                    // Prova a leggere il token dai cookie
                    if let Some(cookie) = req.cookie(state.settings.cookie_jwt_name.as_str()) {	
                        debug!("JWT token found in cookie");
                        Some(cookie.value().to_string())
                    } else {
                        warn!("Authorization header missing and no JWT token found in cookies.");
                        None // Header non presente e cookie non trovato
                    }
                }
            };

            // 3. Valida il Token (se presente)
            match token {
                Some(token_str) => {
                    debug!("Attempting to validate JWT...");
                    // Crea la chiave di decodifica
                    let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());
                    // Configura la validazione (controlla scadenza, algoritmo)
                    // Validation::new accetta l'algoritmo che ti aspetti
                    let validation = Validation::new(Algorithm::HS256);
                    // Potresti aggiungere altre validazioni (issuer, audience) se le usi
                    // validation.set_audience(&["my_app"]);

                    match decode::<Claims>(&token_str, &decoding_key, &validation) {
                        Ok(token_data) => {
                            debug!(
                                "JWT validation successful. Account ID: {}",
                                token_data.claims.sub
                            );
                            // --- Token Valido ---
                            // Estrai l'ID utente dai claims
                            let user_id = token_data.claims.sub;

                            // (Opzionale ma utile) Inserisci l'utente autenticato
                            // nelle estensioni della richiesta per gli handler successivi
                            let authenticated_user = AuthenticatedAccount { account_id: user_id };
                            req.extensions_mut().insert(authenticated_user);

                            debug!("Authenticated user ID inserted into request extensions");

                            // Chiama il servizio successivo nella catena
                            let fut = service.call(req);

                            // The status method does not exist on the future.
                            // If you need the response status, log it after awaiting the future.

                            fut.await
                        }
                        Err(e) => {
                            warn!("JWT validation failed: {}", e);
                            // --- Token Invalido (scaduto, firma errata, etc.) ---
                            // Restituisci un errore Unauthorized specifico dell'API
                            let api_err = ApiError::Unauthorized(format!("Invalid token: {}", e));
                            // Converti ApiError in ActixError e restituiscilo
                            Err(ActixError::from(api_err))
                        }
                    }
                }
                None => {
                    warn!("Authorization header missing or invalid.");
                    // --- Token Mancante ---
                    // Restituisci un errore Unauthorized
                    let api_err =
                        ApiError::Unauthorized("Authorization token required".to_string());
                    Err(ActixError::from(api_err))
                }
            }
        }) // Fine Box::pin
    } // Fine call
} // Fine impl Service
