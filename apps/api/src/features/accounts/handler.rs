// IN: apps/rest_api/src/features/accounts/handler.rs
use crate::{errors::ApiError, features::accounts, middleware::auth::AuthenticatedAccount, state::AppState};
use actix_web::{get, web, HttpMessage, HttpResponse, Responder, Result as ActixResult};
use tracing::{debug, error, info};

use super::dto::AccountResponse;

#[utoipa::path(
    get,
    path = "/api/v1/accounts/@me",
    responses(
        (status = 200, description = "Current account profile", body = AccountResponse), 
        (status = 401, description = "Unauthorized - Invalid or missing token", body = crate::features::auth::dto::ErrorResponse) // Riferisciti al DTO di errore auth
    ),
    security( 
        ("bearer_auth" = [])
    ),
    tags = ["Accounts"]
)]
#[get("/@me")] 
pub async fn get_current_account_handler(
    req: actix_web::HttpRequest, 
    state: web::Data<AppState>,
) -> ActixResult<impl Responder, ApiError> {
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

        match core_lib::persistence::account_repo::find_by_id(&state.db, &record_id).await {
            Ok(Some(account)) => {
                let response_dto = accounts::dto::AccountResponse::from(account);
                debug!("AccountResponse DTO: {:?}", response_dto);
                Ok(HttpResponse::Ok().json(response_dto))
            }
            Ok(None) => {
                // Questo non dovrebbe succedere se il token è valido e l'utente esiste
                error!(
                    "Account ID {} from valid token not found in DB!",
                    auth_account.account_id
                );
                Err(ApiError::NotFound {
                    resource: "Account".to_string(),
                    id: auth_account.account_id,
                })
            }
            Err(core_err) => {
                error!("DB error fetching account {}: {}", auth_account.account_id, core_err);
                Err(ApiError::from(core_err))
            }
        }
    } else {
        // Questo non dovrebbe accadere se il middleware è configurato correttamente
        // perché il middleware dovrebbe restituire 401 prima di arrivare qui.
        // Ma è bene gestire il caso per robustezza.
        error!(
            "AuthenticatedAccount not found in request extensions after Authentication middleware."
        );
        Err(ApiError::InternalServer(
            "Authentication context missing".to_string(),
        ))
    }
}