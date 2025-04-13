use actix_web::cookie::Key;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

use core_lib::config_models::DatabaseSettings;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DiscordOauthSettings {
    pub client_id: String,
    pub client_secret: String, 
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
    pub discord_oauth: DiscordOauthSettings,
    pub log_level: String, 
    pub jwt_secret: String,
    pub jwt_expiration_hours: Option<i64>,
    pub cookie_jwt_name: String, 
    pub cookie_secret: String, 
    pub bot_api_key: String,
}

pub fn load() -> Result<Settings, ConfigError> {
    let config_path = env::current_dir().unwrap().join("config");
    let environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".into());
    let env_config_file = format!("{}.toml", environment);

    let settings_builder = Config::builder()
        .add_source(File::from(config_path.join("default.toml")).required(true))
        .add_source(File::from(config_path.join(env_config_file)).required(false))
        .add_source(Environment::with_prefix("APP").separator("__"));

    let settings = settings_builder.build()?;

    settings.try_deserialize::<Settings>()
}
