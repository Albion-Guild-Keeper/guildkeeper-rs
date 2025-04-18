use actix_web::{cookie, get, post, web, HttpMessage, HttpResponse, Responder, Result as ActixResult};
use anyhow::Ok;
use core_lib::models::user::{self, User};
use tracing::{debug, error, info};

use crate::{errors::ApiError, middleware::auth::AuthenticatedAccount, state::AppState};
use cookie::Cookie;

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
#[get("/@me/guilds")]
pub async fn get_user_guilds(
    req: actix_web::HttpRequest,
    state: web::Data<AppState>,
) -> ActixResult<impl Responder, ApiError> {
    let authenticated_account = req.extensions().get::<AuthenticatedAccount>().cloned();

    if let Some(auth_account) = authenticated_account {
        info!(
            "Fetching profile for authenticated account ID: {}",
            auth_account.account_id
        );

        let account_id: surrealdb::sql::Thing =
            match core_lib::models::account::parse_id_from_string(&auth_account.account_id) {
                std::result::Result::Ok(id) => {
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


        let users_result = core_lib::persistence::users_repo::get_guilds_by_account_id(&state.db, account_id.to_string().as_str()).await;

        match users_result {
            std::result::Result::Ok(guilds) => {
            if !guilds.is_empty() {
                // Convert the guilds vector to JSON and return it
                std::result::Result::Ok(HttpResponse::Ok().json(guilds))
            } else {
                debug!("No guilds found for the user.");
                std::result::Result::Ok(HttpResponse::NotFound().body("No guilds found for the user."))
            }
            }
            Err(e) => {
            error!("Failed to fetch guilds: {}", e);
            Err(ApiError::InternalServer(
                "Failed to fetch guilds".to_string(),
            ))
            }
        }
    } else {
        error!(
            "AuthenticatedAccount not found in request extensions after Authentication middleware."
        );
        Err(ApiError::InternalServer(
            "Authentication context missing".to_string(),
        ))
    }
}


#[utoipa::path(
    post,
    path = "/api/v1/users/{user_id}",
    responses(
        (status = 200, description = "User created successfully", body = crate::features::users::dto::UserResponse), 
        (status = 400, description = "Bad Request - Invalid input", body = crate::features::auth::dto::ErrorResponse), // Riferisciti al DTO di errore auth
        (status = 401, description = "Unauthorized - Invalid or missing token", body = crate::features::auth::dto::ErrorResponse) // Riferisciti al DTO di errore auth
    ),
    security( 
        ("bearer_auth" = [])
    ),
    tags = ["Users"]
)]
#[post("/{user_id}")]
pub async fn guild_member_addition_handler(
    path: web::Path<String>, 
    query: web::Query<super::dto::CreateUserQuery>,
    state: web::Data<AppState>,
) -> ActixResult<impl Responder, ApiError> {
    let user_id = path.into_inner();
    let guild_id = query.guild_id.clone();

    // 1. Find if the user is already in the DB
    let existed_user = core_lib::persistence::users_repo::find_by_user_id_into_guild(&state.db, &user_id, &guild_id).await;
    // 2. If not, create a new user
    
    // 3. Find if the guild exists 
    // 4. Add Relation to the guild
    // 5. Find if accounts exists with this user_id
    // 6. If yes, add the relation user to the account

    debug!("User found: {:?}", existed_user);

    std::result::Result::Ok(HttpResponse::Ok().finish())
}
