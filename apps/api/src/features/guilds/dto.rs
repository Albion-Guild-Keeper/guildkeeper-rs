use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Debug, Clone)]
#[schema(description = "Represents a guilds list in the system.")]
pub struct GuildsListResponse {
    #[schema(example = json!([
        {
            "id": "...",
        },
        {
            "id": "...",
        },
    ]))]
    pub guilds: Vec<GuildResponse>,
}

#[derive(Serialize, ToSchema, Debug, Clone)]
#[schema(description = "Represents a guild in the system.")]
pub struct GuildResponse {
    pub id: String, 
    #[schema(example = "Albion Guild Keeper")]
    pub name: String,
    #[schema(example = "example-id01")]
    pub icon: Option<String>,
    #[schema(example = "1000000")]
    pub balance: u32,
    #[schema(example = "true")]
    pub application_open: bool,
    #[schema(example = "2023-01-01T12:00:00Z")]
    pub created_at: String,
}