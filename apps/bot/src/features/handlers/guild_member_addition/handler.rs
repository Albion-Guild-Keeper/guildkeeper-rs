use serde_json::json;
use serenity::model::prelude::Member;
use serenity::prelude::*;

use super::dto::User;

// Gestisce l'evento GuildMemberAdd (quando un utente si unisce a un server).
pub async fn guild_member_addition(_ctx: Context, new_member: Member) {
    // Ottieni i dati reali del server
    let user_data = User {
        username: new_member.user.name,
    };

    let err = "no err";
    let response_text = "no response text";
    let result: Result<String, String> = Ok("test".to_owned()); // Changed to Some(true) to match Option type

    match result {
        Ok(response) => {
            println!("API call successful. Response: {}", response_text);
        },
        Err(err) => {
            println!("API call failed: {:?}", err);
        }
    }
}