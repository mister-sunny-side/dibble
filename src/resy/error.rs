use thiserror::Error;

pub type ResyResult<T> = Result<T, ResyError>;

#[derive(Error, Debug)]
pub enum ResyError {
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Reservation slot not available")]
    SlotNotAvailable,

    #[error("Reservation slot already taken")]
    SlotTaken,

    #[error("Invalid booking token")]
    InvalidBookingToken,

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),
}
