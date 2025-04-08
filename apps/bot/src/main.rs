use anyhow::Context as _;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use std::env;

mod commands;
mod api;
mod models;
mod handlers;
mod utils;


#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let intents = GatewayIntents::all();

    let client = Client::builder(&token, intents)
        .event_handler(handlers::Handler)
        .await
        .expect("Error creating client");

    Ok(client.into())
}