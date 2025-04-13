use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
#[schema(description = "Represents a request to add a guild member.")]
pub struct GuildMemberAdditionQuery {
    pub username: String,
}

#[derive(Serialize, ToSchema, Debug, Clone)]
#[schema(description = "Represents a users list in the system.")]
pub struct UsersListResponse {
    #[schema(example = json!([
        {
            "id": "...",
        },
        {
            "id": "...",
        },
    ]))]
    pub users: Vec<UserResponse>,
}

#[derive(Serialize, ToSchema, Debug, Clone)]
#[schema(description = "Represents a user in the system.")]
pub struct UserResponse {
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