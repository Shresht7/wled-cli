use thiserror::Error;

#[derive(Error, Debug)]
pub enum WledError {
    #[error("API call failed: {0}")]
    ApiError(String),

    #[error("HTTP request failed")]
    HttpError(#[from] reqwest::Error),

    #[error("Failed to parse value")]
    ParseError(#[from] std::num::ParseIntError),
}

pub type Result<T> = std::result::Result<T, WledError>;
