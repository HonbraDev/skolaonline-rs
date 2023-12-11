use std::collections::BTreeMap;

use lazy_static::lazy_static;
use url::Url;

use crate::{
    client::{BodyType, Client},
    error::Error,
    model::{min_app_version::MinAppVersion, token_connect::TokenResponse, user_info::UserInfo},
};

lazy_static! {
    pub static ref BASE_URL: Url = Url::parse("https://aplikace.skolaonline.cz/solapi/api/")
        .expect("API base URL should be valid");
    pub static ref BASE_URL_V1: Url = BASE_URL.join("v1/").expect("API v1 URL should be valid");
}

impl Client {
    pub async fn sign_in(&self, username: &str, password: &str) -> Result<TokenResponse, Error> {
        let mut body = BTreeMap::new();
        body.insert("grant_type", "password");
        body.insert("username", username);
        body.insert("password", password);
        body.insert("client_id", "test_client");
        body.insert("scope", "openid offline_access profile sol_api");
        self.post(
            BASE_URL.join("connect/token")?,
            None,
            Some((&body, BodyType::Form)),
        )
        .await
    }

    pub async fn get_min_app_version(&self) -> Result<MinAppVersion, Error> {
        self.get(BASE_URL_V1.join("minAppVersion")?, None).await
    }

    pub async fn get_user_info(&self, token: &str) -> Result<UserInfo, Error> {
        self.get(BASE_URL_V1.join("user")?, Some(token)).await
    }
}
