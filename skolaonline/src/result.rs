use thiserror::Error;

use crate::response::APIResponseStatus;

pub type SOResult<T> = Result<T, SOError>;

#[derive(Error, Debug)]
pub enum SOError {
    /// The server responded with a non-success status code
    /// and provided a status response
    #[error("non-success API status: {0}")]
    Status(APIResponseStatus),

    /// The server responded with a non-success status code
    /// and did not provide a status response (or it was malformed)
    #[error("non-success API status: {0} (no further information provided)")]
    BadStatus(u16),

    /// The server responded with a success status code
    /// but the response could not be decoded
    #[error("failed to decode response: {0}")]
    Decode(reqwest::Error),

    /// An error has occured while sending/receiving the request
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
}
