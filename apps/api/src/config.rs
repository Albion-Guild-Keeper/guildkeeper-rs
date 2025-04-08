use actix_web::cookie::Key;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

use core_lib::config_models::DatabaseSettings;

// Definisci le struct specifiche SOLO per l'API qui
#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DiscordOauthSettings {
    pub client_id: String,
    pub client_secret: String, // SECRET! Caricato da Env Var
    pub redirect_uri: String,
}

// Definisci la struct COMPLETA per l'API
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    // Usa il tipo condiviso per il database
    pub database: DatabaseSettings,

    // Usa i tipi specifici definiti sopra
    pub server: ServerSettings,
    pub discord_oauth: DiscordOauthSettings,

    // Altri campi specifici dell'API
    pub log_level: String,  // Anche se comune, la leggiamo qui
    pub jwt_secret: String, // SECRET! Caricato da Env Var
    pub jwt_duration_hours: Option<i64>, // Opzionale, default 1 ora
    pub cookie_secret: String, // SECRET! Caricato da Env Var
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
