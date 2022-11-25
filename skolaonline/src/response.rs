use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct APIResponse<T>{
    pub status: APIResponseStatus,
    pub data: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct APIResponseStatus {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}
