use actix_web::{get, web, HttpMessage, HttpResponse, Responder, Result as ActixResult};
use tracing::{error, info};

use crate::{errors::ApiError, middleware::auth::AuthenticatedAccount, state::AppState};

#[utoipa::path(
    get,
    path = "/api/v1/guilds/@me",
    responses(
        (status = 200, description = "Current guilds list", body = crate::features::guilds::dto::GuildsListResponse), 
        (status = 401, description = "Unauthorized - Invalid or missing token", body = crate::features::auth::dto::ErrorResponse) // Riferisciti al DTO di errore auth
    ),
    security( 
        ("bearer_auth" = [])
    ),
    tags = ["Guilds"]
)]
#[get("/@me")]
pub async fn get_guildlist(
    req: actix_web::HttpRequest, 
    state: web::Data<AppState>,
) ->  ActixResult<impl Responder, ApiError> {
    let authenticated_account = req.extensions().get::<AuthenticatedAccount>().cloned(); // Clona se trovato

    if let Some(auth_account) = authenticated_account {
        info!(
            "Fetching profile for authenticated account ID: {}",
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

        let guilds = core_lib::persistence::guilds_repo::find_by_account_id(
            &state.db,
            &record_id.to_string(),
        )
        .await
        .map_err(|e| {
            error!("Error fetching guilds: {:?}", e);
            ApiError::InternalServer("Failed to fetch guilds".to_string())
        })?;

        info!("Fetched guilds: {:?}", guilds);
        Ok(HttpResponse::Ok().json(guilds))
    } else {
        error!("No authenticated account found in request extensions");
        Err(ApiError::Unauthorized("No authenticated account found".to_string()))
    }
} 