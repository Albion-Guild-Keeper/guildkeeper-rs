use actix_web::web;

use crate::middleware::auth::Authentication;

pub mod auth;
pub mod users;

// 2. Definisci la funzione pubblica `register_routes`
//    Prende una configurazione di servizio (`ServiceConfig`) modificabile da Actix.
pub fn register_routes(cfg: &mut web::ServiceConfig) {
    // 3. Chiama le funzioni `register_routes` (o un nome simile)
    //    definite all'interno di CIASCUN modulo feature.
    //    Puoi usare `web::scope` per raggruppare ulteriormente le route
    //    di una feature sotto un prefisso comune (relativo allo scope
    //    già definito in main.rs, es. /api/v1).

    cfg.service(
        // Aggiunge le route della feature 'auth' sotto lo scope corrente
        web::scope("/auth") // Le route auth saranno sotto /api/v1/auth/...
            .configure(auth::register_routes), // Delega a auth/mod.rs
    );

    // Aggiungi altre feature qui, ad esempio:
    cfg.service(
        web::scope("/users")
            .wrap(Authentication)
            .configure(users::register_routes),
    ); // Aggiungi la feature users
}
