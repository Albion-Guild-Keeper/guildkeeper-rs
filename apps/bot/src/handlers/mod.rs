pub mod message;
pub mod interaction_create;
pub mod ready;
pub mod guild_create;
pub mod guild_member_addition;

use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        message::handle_message(ctx, msg).await;
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create::handle_interaction(ctx, interaction).await;
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready).await;
    }
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        guild_create::handle_guild_create(ctx, guild, is_new).await;
    }
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        guild_member_addition::handle_guild_member_add(ctx, new_member).await;
    }
}