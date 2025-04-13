use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::features::commands::*;

pub async fn handle_interaction(ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        
        let commands = handle_commands(&command).await;

        let builder = CreateInteractionResponse::Message(commands);

        if let Err(why) = command.create_response(&ctx.http, builder).await {
            eprintln!("Error responding to slash command '{}': {why}", command.data.name);
        }
    }
}

async fn handle_commands(command: &CommandInteraction) -> CreateInteractionResponseMessage {
    match command.data.name.as_str() {
        "panel" => panel::run(&command.data.options()).await,
        "ping" => ping::run(&command.data.options()),
        _ => CreateInteractionResponseMessage::new().content("not implemented :("),
    }
}