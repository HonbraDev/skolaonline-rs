use crate::response::APIResponse;
use anyhow::{anyhow, Result};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};

/// A client for the Škola OnLine API
pub struct SOClient {
    http_client: Client,
    base_path: &'static str,
    pub username: String,
}

fn basic_auth(username: &str, password: &str) -> HeaderMap {
    HeaderMap::from_iter([(HeaderName::from_static("authorization"), {
        HeaderValue::try_from(format!(
            "Basic {}",
            base64::encode(format!("{username}:{password}"))
        ))
        .unwrap()
    })])
}

impl SOClient {
    /// Creates a new client for the Škola OnLine API with the given credentials
    ///
    /// Errors if:
    /// - reqwest fails to create a client
    /// - reqwest fails to set the authorization header
    pub fn new(username: &str, password: &str) -> Self {
        let client = Client::builder()
            .default_headers(basic_auth(username, password))
            .user_agent("Samsung Smart Fridge <honbra@honbra.com>")
            .build()
            .unwrap();
        Self {
            http_client: client,
            base_path: "https://aplikace.skolaonline.cz/SOLWebApi/api/v1",
            username: username.to_string(),
        }
    }
}

/// Handles the raw response and converts it into `APIResponse<T>`
async fn handle_response<T>(response: reqwest::Response) -> Result<APIResponse<T>>
where
    T: serde::de::DeserializeOwned,
{
    let status = response.status();
    let is_success = status.is_success();
    let response = response.json().await;

    match response {
        Ok(response) => {
            if is_success {
                Ok(response)
            } else {
                Err(anyhow!(response.status.message))
            }
        }
        Err(e) => {
            if is_success {
                Err(anyhow!("response was not valid JSON: {}", e))
            } else {
                Err(anyhow!("request failed with status {status}"))
            }
        }
    }
}

impl SOClient {
    // why
    fn get_url(&self, path: &str) -> String {
        format!("{}{path}", self.base_path)
    }
    
    /// Executes a GET request to the given path and returns the response
    pub async fn get<T>(&self, url: &str) -> Result<APIResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.http_client.get(self.get_url(url)).send().await?;

        handle_response(response).await
    }

    /// Executes a POST request to the given path and returns the response
    pub async fn post<T, B>(&self, url: &str, body: Option<B>) -> Result<APIResponse<T>>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let response = self
            .http_client
            .post(self.get_url(url))
            .json(&body)
            .send()
            .await?;

        handle_response(response).await
    }
}
