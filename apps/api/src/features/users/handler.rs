use actix_web::{get, web, HttpMessage, HttpResponse, Responder, Result as ActixResult};
use core_lib::models::user::User;
use tracing::{error, info};

use crate::{errors::ApiError, middleware::auth::AuthenticatedAccount, state::AppState};

#[utoipa::path(
    get,
    path = "/api/v1/users/@me",
    responses(
        (status = 200, description = "Current user profile", body = crate::features::users::dto::UserResponse), 
        (status = 401, description = "Unauthorized - Invalid or missing token", body = crate::features::auth::dto::ErrorResponse) // Riferisciti al DTO di errore auth
    ),
    security( 
        ("bearer_auth" = [])
    ),
    tags = ["Users"]
)]
#[get("/@me")]
pub async fn guild_member_addition_handler(
    query: web::Query<super::dto::GuildMemberAdditionQuery>,
    req: actix_web::HttpRequest,
    state: web::Data<AppState>,
) -> ActixResult<impl Responder, ApiError> {
    let authenticated_account = req.extensions().get::<AuthenticatedAccount>().cloned();

    if let Some(auth_account) = authenticated_account {
        info!(
            "Adding member for authenticated account ID: {}",
            auth_account.account_id
        );

        let account_id: surrealdb::sql::Thing =
            match core_lib::models::account::parse_id_from_string(&auth_account.account_id) {
                Ok(id) => {
                    info!("Parsed account ID: {:?}", id);
                    id
                }
                Err(_) => {
                    error!("Invalid account ID format found in token");
                    return Err(ApiError::InternalServer(
                        "Invalid account ID format found in token".to_string(),
                    ));
                }
            };

        let record_id: surrealdb::RecordId = account_id
            .to_string()
            .parse()
            .expect("Failed to parse RecordId");

        info!("Record ID: {:?}", record_id);

        let username_query = &query.username;

        match core_lib::persistence::users_repo::create(
            &state.db,
            User {
                id: User::default_id(),
                username: username_query.clone()
            },
        )
        .await
        {
            Ok(user) => {
                info!("User created: {:?}", user);
            }
            Err(e) => {
                error!("Error creating user: {:?}", e);
                return Err(ApiError::InternalServer(
                    "Failed to create user".to_string(),
                ));
            }
        }

        // ! CHANGE HERE
        Ok(HttpResponse::Ok().json(super::dto::UsersListResponse {
            users: vec![super::dto::UserResponse {
                id: record_id.to_string(),
                name: username_query.clone(),
                icon: None,
                balance: 0,
                application_open: false,
                created_at: "2023-01-01T12:00:00Z".to_string(),
            }],
        }))
    } else {
        error!("No authenticated account found in request extensions");
        Err(ApiError::Unauthorized("No authenticated account found".to_string()))
    }
}
