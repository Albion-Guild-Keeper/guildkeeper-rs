use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,           // Es. "ws://localhost:8000" o "file:///path/to/db"
    pub namespace: Option<String>, // Es. "my_namespace"
    pub database_name: Option<String>, // Es. "my_database"
    pub username: Option<String>, // Es. "my_user"
    pub password: Option<String>, // Es. "my_password"
}