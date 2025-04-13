use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServicesError {
    #[error("Request error: {0}")]
    RequestError(reqwest::Error),
    #[error("Response error: {0}")]
    ResponseError(String),
    #[error("JSON parsing error: {0}")]
    DeserializeError(serde_json::Error),
    #[error("Serialization error: {0}")]
    SerializeError(serde_json::Error),
    #[error("Unknown error occurred")]
    UnknownError,
}