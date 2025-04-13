pub mod guild_member_addition;
pub mod interaction_create;
pub mod ready;
pub mod guild_create;

use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready).await;
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create::handler::handle_interaction(ctx, interaction).await;
    }
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        guild_member_addition::handler::guild_member_addition(ctx, new_member).await;
    }
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        guild_create::handler::handle_guild_create(ctx, guild, is_new).await;
    }
}
