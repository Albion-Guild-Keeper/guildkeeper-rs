use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{web::{self, scope, Data}, cookie::Key, App, HttpServer}; // Import Key, App, HttpServer
use state::AppState;
use tracing::info;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod errors;
mod features;
mod state;
mod middleware;

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
    info!("Using cookie secret with length: {}", settings.cookie_secret.len());
    let session_key = Key::derive_from(settings.cookie_secret.as_bytes());

    // Costruisci la configurazione delle route passando lo stato
    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(Data::new(app_state.clone()))
            .service(
                scope("/api/v1")
                .wrap(
                    // Use SessionMiddleware::new instead of builder if you don't need complex builder config
                    SessionMiddleware::builder(
                        CookieSessionStore::default(), // Usa CookieSessionStore per Shuttle
                        session_key.clone() // Clone the key for the closure
                    )
                    .cookie_secure(true) // In Shuttle dovresti sempre usare HTTPS
                    .cookie_http_only(true)
                    .cookie_same_site(actix_web::cookie::SameSite::Lax) // Considera `cookie_domain` se hai un dominio custom con Shuttle
                    .build() 
                )
                .configure(features::register_routes), // Assumi che questo registri anche SwaggerUI e OpenAPI JSON
            );
        // Registra SwaggerUI qui se non lo fai in features::register_routes
        let openapi = crate::ApiDoc::openapi(); // Assumi ApiDoc sia definita in main o importata
        cfg.service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", openapi.clone()),
        );
    };

    Ok(config.into()) // Usa .into() per convertire nel tipo atteso da ShuttleActixWeb
}
