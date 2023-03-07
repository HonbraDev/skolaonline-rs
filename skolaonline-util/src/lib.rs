use std::string;

use base64::prelude::{Engine as _, BASE64_STANDARD};
use thiserror::Error;

pub fn basic_auth_encode(username: &str, password: &str) -> String {
    BASE64_STANDARD.encode(format!("{username}:{password}"))
}

#[derive(Debug, Error)]
pub enum BasicAuthDecodeError {
    #[error("value is not valid base64")]
    InvalidBase64(#[from] base64::DecodeError),

    #[error("encoded value is not UTF-8")]
    InvalidUtf8(#[from] string::FromUtf8Error),

    #[error("invalid value")]
    InvalidValue,
}

// Source: https://github.com/EstebanBorai/http-auth-basic/blob/4d2d5f871c72fc5df93fc82a0c1773f5587eeae3/src/credentials.rs#L36
pub fn basic_auth_decode(encoded: &str) -> Result<(String, String), BasicAuthDecodeError> {
    let decoded = BASE64_STANDARD.decode(encoded)?;
    let as_utf8 = String::from_utf8(decoded)?;

    if let Some((user_id, password)) = as_utf8.split_once(':') {
        return Ok((user_id.to_string(), password.to_string()));
    }

    Err(BasicAuthDecodeError::InvalidValue)
}
