use serenity::prelude::*;
use serenity::model::guild::Guild;
use crate::api::fetch::{fetch_data, FetchType};
use serde_json::json;
use crate::models::discord::Discord;

// Gestisce l'evento GuildCreate (quando il bot viene aggiunto a un server).
pub async fn handle_guild_create(ctx: Context, guild: Guild, _is_new: Option<bool>) {
    // Trova il canale predefinito (o un canale specifico).
    if let Some(channel_id) = guild.system_channel_id {
        // Invia il messaggio di saluto.
        let _ = channel_id.say(&ctx.http, "Ciao a tutti! Sono un bot.").await;
    }

    // Ottieni i dati reali del server
    let discord_data = Discord {
        id: guild.id.get() as i64, // ID del server
        discord_name: guild.name, // Nome del server
        // joined_at: Some(chrono::Utc::now().to_rfc3339()), // Data e ora correnti
        joined_at: guild.joined_at.to_string() // Data e ora di unione al server
    };
    
    // Construct the URL
    let url = format!("http://127.0.0.1:8000/api/v1/guild_create/{}", discord_data.id);
    // Make the PUT request using fetch_data (synchronous)
    let result = fetch_data(FetchType::PUT, &url, Some(json!({
        "id": discord_data.id,
        "discord_name": discord_data.discord_name,
        "joined_at": discord_data.joined_at
    })));

    match result.await {
        Ok(response_text) => {
            println!("API call successful. Response: {}", response_text);
        },
        Err(err) => {
            println!("API call failed: {:?}", err);
        }
    }
}