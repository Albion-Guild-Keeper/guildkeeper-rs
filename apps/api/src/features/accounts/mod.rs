use actix_web::web;

pub mod dto;
pub mod handler;

use self::handler::get_current_account_handler;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_current_account_handler); // Usa la macro #[get("/@me")]
}
