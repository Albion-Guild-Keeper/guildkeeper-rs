use serde::{Deserialize, Serialize};
use serenity::prelude::*;
use serenity::model::guild::Guild as SerenityGuild;
use reqwest;

#[derive(Serialize, Deserialize, Debug)]
struct Guild {
    id: u64,
    name: String,
    owner_id: u64,
}

// Gestisce l'evento GuildCreate (quando il bot viene aggiunto a un server).
pub async fn handle_guild_create(ctx: Context, guild: SerenityGuild, _is_new: Option<bool>) {
    // Trova il canale predefinito (o un canale specifico).
    if let Some(channel_id) = guild.system_channel_id {
        // Invia il messaggio di saluto.
        let _ = channel_id.say(&ctx.http, "Ciao a tutti! Sono un bot.").await;
    }

    let guild_data = Guild {
        id: guild.id.get(),
        name: guild.name.clone(),
        owner_id: guild.owner_id.get(),
    };

    println!("Sending guild data: {:?}", guild_data);

    let url = format!("http://127.0.0.1:8000/api/v1/guild_create");
    
    let client = reqwest::Client::new();
    let result = client.put(&url)
        .json(&guild_data)
        .send()
        .await;

    match result {
        Ok(response) => {
            let response_text = response.text().await.unwrap_or_else(|_| String::from("No content"));
            println!("API call successful. Response: {}", response_text);
        },
        Err(err) => {
            println!("API call failed: {:?}", err);
        }
    }
}