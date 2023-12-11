use reqwest::Method;
use url::Url;

use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) http_client: reqwest::Client,
}

impl Client {
    pub fn new(user_agent: &str) -> Result<Self, reqwest::Error> {
        Ok(Self {
            http_client: reqwest::Client::builder().user_agent(user_agent).build()?,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyType {
    Json,
    Form,
}

impl Client {
    pub async fn execute_request<TReq, TRes>(
        &self,
        method: Method,
        url: Url,
        token: Option<&str>,
        body: Option<(&TReq, BodyType)>,
    ) -> Result<TRes, Error>
    where
        TReq: serde::Serialize + ?Sized,
        TRes: serde::de::DeserializeOwned,
    {
        let mut request = self.http_client.request(method, url);
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        if let Some((body, body_type)) = body {
            request = match body_type {
                BodyType::Json => request.json(body),
                BodyType::Form => request.form(body),
            }
        }
        let response = request.send().await?;
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            println!("{body}");
            return Err(Error::Status(status));
        }
        Ok(response.json().await?)
    }

    pub async fn get<T>(&self, url: Url, token: Option<&str>) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        self.execute_request::<(), T>(Method::GET, url, token, None)
            .await
    }

    pub async fn post<TReq, TRes>(
        &self,
        url: Url,
        token: Option<&str>,
        body: Option<(&TReq, BodyType)>,
    ) -> Result<TRes, Error>
    where
        TReq: serde::Serialize + ?Sized,
        TRes: serde::de::DeserializeOwned,
    {
        self.execute_request(Method::POST, url, token, body).await
    }
}
