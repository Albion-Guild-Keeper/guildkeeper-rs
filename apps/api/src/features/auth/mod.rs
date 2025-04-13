use actix_web::web;

pub mod handler;
pub mod dto;
pub mod service;
pub mod client;

use self::handler::{discord_login_handler, discord_callback_handler, discord_logout_handler};

// Funzione chiamata da features::register_routes
pub fn register_routes(cfg: &mut web::ServiceConfig) {
    // Associa i path relativi allo scope /auth agli handler
    cfg.service(discord_login_handler); // Usa la macro #[get("/login")]
    cfg.service(discord_callback_handler); // Usa la macro #[get("/callback")]
    cfg.service(discord_logout_handler); // Usa la macro #[get("/logout")]
     // Aggiungi .service() per altri handler in questo modulo/feature
}