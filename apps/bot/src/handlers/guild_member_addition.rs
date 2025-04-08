use crate::api::fetch::{fetch_data, FetchType};
use crate::models::user::User;
use serde_json::json;
use serenity::model::prelude::Member;
use serenity::prelude::*;

// Gestisce l'evento GuildMemberAdd (quando un utente si unisce a un server).
pub async fn handle_guild_member_add(_ctx: Context, new_member: Member) {
    // Trova il canale predefinito (o un canale specifico).

    println!("A New User Joined INFO: {:#?}", new_member);

    // Ottieni i dati reali del server
    let user_data = User {
        id: new_member.user.id.get() as i64,
        joined_at: new_member.joined_at.unwrap().to_string(),
        username: new_member.user.name.clone(),
        discord_id: new_member.guild_id.get() as i64,
        server_name: new_member.user.global_name.unwrap_or_default(),
    };

    // Construct the URL
    let url = format!("http://127.0.0.1:8000/api/v1/user/{}", user_data.id);
    // Make the PUT request using fetch_data (synchronous)
    let result = fetch_data(FetchType::PUT, &url, Some(json!({
        "id": user_data.id,
        "joined_at": user_data.joined_at,
        "username": user_data.username,
        "server_name": user_data.server_name,
        "discord_id": user_data.discord_id
    })));
    println!("URL: {}", user_data.discord_id);

    match result.await {
        Ok(response_text) => {
            println!("API call successful. Response: {}", response_text);
        },
        Err(err) => {
            println!("API call failed: {:?}", err);
        }
    }
}
