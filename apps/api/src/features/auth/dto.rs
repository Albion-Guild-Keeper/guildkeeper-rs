use serde::{Serialize, Deserialize};
use utoipa::{ToSchema, IntoParams};

#[derive(Deserialize, IntoParams)]
pub struct CallbackQuery {
    pub code: String,
    pub state: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    #[schema(example = "ey...")]
    pub access_token: String,
    #[schema(example = "Bearer")]
    pub token_type: String,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    #[schema(example = "Authentication Failed")]
    pub error: String,
    #[schema(example = "Invalid state parameter")]
    pub message: Option<String>,
}

// Claims / Dati che finiscono nel payload del JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
