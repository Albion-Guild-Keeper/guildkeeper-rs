use actix_web::web;

pub mod dto;
pub mod handler;

use self::handler::{guild_member_addition_handler, get_user_guilds};

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_guilds);
    cfg.service(guild_member_addition_handler); // ! @todo da mettere in public/ quando sarà pronto
}
