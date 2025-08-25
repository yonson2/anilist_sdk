use thiserror::Error;

#[derive(Error, Debug)]
pub enum AniListError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("GraphQL error: {message}")]
    GraphQL { message: String },

    #[error("Rate limited")]
    RateLimit,

    #[error("Not found")]
    NotFound,
}
