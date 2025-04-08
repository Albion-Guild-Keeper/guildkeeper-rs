use actix_web::web;

pub mod client;
pub mod dto;
pub mod handler;
pub mod service;

use self::handler::{create_user_handler, get_user_handler};

// Funzione chiamata da features::register_routes
pub fn register_routes(cfg: &mut web::ServiceConfig) {
    // Associa i path relativi allo scope /auth agli handler
    cfg.service(create_user_handler); // Usa la macro #[get("/login")]
    cfg.service(get_user_handler); // Usa la macro #[get("/callback")]
                                   // Aggiungi .service() per altri handler in questo modulo/feature
}
