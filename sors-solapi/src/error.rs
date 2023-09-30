use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("Non-success status code: {0}")]
    Status(StatusCode),
    #[error("Failed to parse URL: {0}")]
    UrlParse(#[from] url::ParseError),
}
