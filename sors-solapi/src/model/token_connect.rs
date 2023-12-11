use std::time::Duration;

use serde::Deserialize;
use serde_with::{formats::SpaceSeparator, serde_as, DurationSeconds, StringWithSeparator};

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    #[serde_as(as = "DurationSeconds<u64>")]
    pub expires_in: Duration,
    pub refresh_token: String,
    #[serde_as(as = "StringWithSeparator::<SpaceSeparator, String>")]
    pub scope: Vec<String>,
    // if this isn't `Bearer`, run.
    pub token_type: String,
}
