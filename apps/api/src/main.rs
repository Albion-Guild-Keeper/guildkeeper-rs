use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    http::header, // Importa le costanti delle intestazioni
    web::{self, scope, Data},
    App, HttpServer,
}; // Import Key, App, HttpServer
use state::AppState;
use std::sync::Arc;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod errors;
mod features;
mod middleware;
mod state;

use core_lib::persistence::db::create_surreal_connection;

// ===>>> Definizione OpenAPI <<<===
#[derive(OpenApi)]
#[openapi(
    // Elenca TUTTI i path handler annotati con #[utoipa::path]
    paths(
        features::auth::handler::discord_login_handler,
        features::auth::handler::discord_callback_handler,
        features::auth::handler::discord_logout_handler,
        features::accounts::handler::get_current_account_handler,
        features::guilds::handler::get_guildlist,
        features::users::handler::guild_member_addition_handler,
        // ... aggiungi TUTTI gli altri handler dell'API ...
    ),
    // Elenca TUTTI i componenti (DTO, risposte, parametri) annotati con #[derive(ToSchema/IntoParams)]
    components(
        schemas(
            features::auth::dto::LoginResponse,
            features::auth::dto::ErrorResponse,
        ),
        // Puoi definire risposte riutilizzabili qui se vuoi
        // responses( ... )
    ),
    // Definisci tag per organizzare le API nella UI di Swagger
    tags(
        (name = "Health", description = "Application Health Endpoint"),
        (name = "Authentication - Discord", description = "Discord OAuth2 Authentication Flow"),
        (name = "Users", description = "User Management Endpoints")
        // ... aggiungi altri tag ...
    ),
    // Info generali dell'API
    info(
        title = "My Awesome Project API",
        version = "1.0.0",
        description = "API endpoints for the awesome project",
        // contact(name = "Support", email = "support@example.com"),
        // license(name = "MIT")
    ),
    // (Opzionale) Definizioni di sicurezza (es. Bearer token per JWT)
    // security(
    //     ("bearer_auth" = [])
    // ),
    // components(
    //    security_schemes(
    //        ("bearer_auth" = utoipa::openapi::security::SecurityScheme::Http(
    //            utoipa::openapi::security::HttpAuthScheme::Bearer.into()
    //        ))
    //    )
    // )
)]
struct ApiDoc; // Struct vuota che serve solo per l'aggregazione OpenAPI

// La funzione main ANNOTATA che chiama la configurazione
#[shuttle_runtime::main]
async fn actix_web(// Inietta risorse Shuttle se necessario, es:
    // #[shuttle_shared_db::Surreal] db: surrealdb::Surreal<surrealdb::engine::any::Any>,
    // #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> shuttle_actix_web::ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static>
{
    dotenvy::dotenv().ok();
    // ... carica settings (prendendo i segreti da `secrets` se usi shuttle-secrets) ...
    let settings = config::load().expect("Failed to load settings"); // Use a more descriptive panic message
                                                                     // ... crea connessione DB (o usa quella iniettata) ...
                                                                     // let db_connection = create_surreal_connection(...).await.expect(...); // O usa `db` iniettata
    let db_connection = create_surreal_connection(&settings.database)
        .await
        .expect("Failed to connect to database"); // Use a more descriptive panic message

    let app_state = AppState {
        db: db_connection,
        settings: Arc::new(settings.clone()),
    };

    if settings.cookie_secret.is_empty() {
        panic!("FATAL: COOKIE_SECRET environment variable not set or empty.");
    }
    info!(
        "Using cookie secret with length: {}",
        settings.cookie_secret.len()
    );
    let session_key = Key::derive_from(settings.cookie_secret.as_bytes());

    // Costruisci la configurazione delle route passando lo stato
    let config = move |cfg: &mut web::ServiceConfig| {
        // Definisci una configurazione CORS più specifica
        let cors = Arc::new(
            Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .supports_credentials()
                .max_age(3600)
                .allowed_origin("http://localhost:8080") // Allow your frontend origin
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]) // Specify allowed methods
                .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT, actix_web::http::header::CONTENT_TYPE]) // Specify allowed headers
        );

        // Define Session Middleware settings
        let session_middleware = SessionMiddleware::builder(
                CookieSessionStore::default(),
                session_key.clone(),
            )
            .cookie_secure(false) // For local HTTP development put this to false
            .cookie_http_only(true) // Prevents JavaScript access to the cookie
            .build();

        // Add shared application state first
        cfg.app_data(Data::new(app_state.clone()));

        // Configure the /api/v1 scope and apply middleware to it
        cfg.service(
            scope("/api/v1")
                // Apply CORS first to the scope, wrapping subsequent middleware and routes
                .wrap(cors)
                // Apply Session Middleware next (wrapped by CORS)
                .wrap(session_middleware)
                // Configure routes within the scope (also wrapped by CORS and Session)
                .configure(features::register_routes),
        );

        // Registra SwaggerUI (outside the /api/v1 scope)
        // Note: Swagger UI will NOT have CORS or Session middleware applied with this structure
        let openapi = crate::ApiDoc::openapi();
        cfg.service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
        );
    };

    Ok(config.into()) // Usa .into() per convertire nel tipo atteso da ShuttleActixWeb
}
