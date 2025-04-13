use actix_web::web;

use crate::middleware::auth::Authentication;

pub mod auth;
pub mod accounts;
pub mod guilds;
pub mod users;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth") 
            .configure(auth::register_routes),
    );
    cfg.service(
        web::scope("/accounts")
            .wrap(Authentication)
            .configure(accounts::register_routes),
    );
    cfg.service(
        web::scope("/guilds")
            .wrap(Authentication)
            .configure(guilds::register_routes),
    );
    cfg.service(
        web::scope("/users")
            .wrap(Authentication)
            .configure(users::register_routes),
    );
}