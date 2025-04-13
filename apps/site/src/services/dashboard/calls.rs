use reqwest::Client;

use crate::{errors::ServicesError, models::dashboard::GuildsListResponse};

const URL: &str = "http://localhost:8010/api/v1";

pub async fn get_guildslist() -> Result<GuildsListResponse, ServicesError> {
    let client = Client::new();
    let response = client
        .get(format!("{}/guilds/@me", URL))
        .send()
        .await
        .map_err(|e| ServicesError::RequestError(e))?;
    if response.status().is_success() {
        let guilds_list = response
            .json::<GuildsListResponse>()
            .await
            .map_err(|e| ServicesError::ResponseError(e.to_string()))?;
        println!("Response: {:?}", guilds_list);
        Ok(guilds_list)
    } else {
        Err(ServicesError::ResponseError(format!(
            "HTTP error: {}",
            response.status()
        )))
    }
}
