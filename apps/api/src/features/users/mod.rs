use actix_web::web;

pub mod dto;
pub mod handler;

use self::handler::guild_member_addition_handler;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(guild_member_addition_handler);
}
