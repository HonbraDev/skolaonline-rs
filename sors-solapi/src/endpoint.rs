use lazy_static::lazy_static;
use url::Url;

use crate::{
    client::Client,
    error::Error,
    model::{min_app_version::MinAppVersion, user_info::UserInfo},
};

lazy_static! {
    pub static ref BASE_URL: Url = Url::parse("https://aplikace.skolaonline.cz/solapi/api/")
        .expect("Failed to parse API base URL");
    pub static ref BASE_URL_V1: Url = BASE_URL.join("v1/").expect("Failed to parse API v1 URL");
}

impl Client {
    pub async fn get_min_app_version(&self) -> Result<MinAppVersion, Error> {
        self.get(BASE_URL_V1.join("minAppVersion")?, None).await
    }

    pub async fn get_user_info(&self, token: &str) -> Result<UserInfo, Error> {
        self.get(BASE_URL_V1.join("user")?, Some(token)).await
    }
}
