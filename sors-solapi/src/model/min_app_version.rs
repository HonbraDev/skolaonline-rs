use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct MinAppVersion {
    #[serde(rename = "iOS")]
    pub ios: String,
    pub android: String,
}
