use actix_web::web;

pub mod dto;
pub mod handler;

use self::handler::get_guildlist;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_guildlist); // Usa la macro #[get("/guilds")]
}
