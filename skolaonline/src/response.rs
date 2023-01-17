use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct APIResponse<T> {
    pub status: APIResponseStatus,
    pub data: T,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct APIResponseStatus {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}
