use http::header::AUTHORIZATION;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

use crate::{
    response::APIResponse,
    result::{SOError, SOResult},
};

/// A client for the Škola OnLine API
pub struct SOClient {
    http_client: Client,
    base_path: &'static str,
    pub username: String,
}

fn basic_auth(username: &str, password: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let auth = format!("Basic {}", base64::encode(format!("{username}:{password}")));
    let auth = HeaderValue::from_str(&auth).unwrap();

    headers.insert(AUTHORIZATION, auth);

    headers
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

/// Handles the raw response and converts it into a `SOResult<T>`
async fn handle_response<T>(response: reqwest::Response) -> SOResult<T>
where
    T: serde::de::DeserializeOwned,
{
    let status = response.status();
    let is_success = status.is_success();
    let response = response.json::<APIResponse<T>>().await;

    match response {
        Ok(response) => {
            if is_success {
                Ok(response.data)
            } else {
                Err(SOError::Status(response.status))
            }
        }
        Err(e) => {
            if is_success {
                Err(SOError::Decode(e))
            } else {
                Err(SOError::BadStatus(status.as_u16()))
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
    pub async fn get<T>(&self, url: &str) -> SOResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.http_client.get(self.get_url(url)).send().await?;

        handle_response(response).await
    }

    /// Executes a POST request to the given path and returns the response
    pub async fn post<T, B>(&self, url: &str, body: Option<B>) -> SOResult<T>
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
