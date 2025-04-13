use anyhow::Context as _;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use std::env;
use tracing::info;

mod features;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let intents = GatewayIntents::all();

    info!("Starting bot with token: {}", token);

    let client = Client::builder(token, intents)
        .event_handler(features::handlers::Handler)
        .await
        .expect("Error creating client");

    Ok(client.into())
}