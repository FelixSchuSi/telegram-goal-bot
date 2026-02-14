use base64::{engine::general_purpose, Engine as _};
use log::error;
use reqwest::header;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AuthResponse {
    access_token: String,
}

#[derive(Debug)]
pub enum RedditError {
    Request(reqwest::Error),
    Status(reqwest::Response),
}

impl From<reqwest::Error> for RedditError {
    fn from(err: reqwest::Error) -> Self {
        RedditError::Request(err)
    }
}

pub async fn refresh_client() -> Result<Client, RedditError> {
    let reddit_client_id = std::env::var("REDDIT_CLIENT_ID").unwrap();
    let refresh_token = std::env::var("REDDIT_REFRESH_TOKEN").unwrap();
    let user_agent = std::env::var("REDDIT_USER_AGENT").unwrap();
    let http_proxy = std::env::var("REDDIT_HTTP_PROXY").unwrap();

    let form = [
        ("grant_type", "refresh_token"),
        ("refresh_token", &refresh_token),
    ];

    let auth_value = general_purpose::STANDARD.encode(format!("{}:", reddit_client_id));
    let request = Client::new()
        .post("https://www.reddit.com/api/v1/access_token")
        .header(USER_AGENT, &user_agent)
        .header("Authorization", format!("Basic {}", auth_value))
        .form(&form);

    let response = request.send().await?;

    if !response.status().is_success() {
        Err(RedditError::Status(response))
    } else {
        let auth_data: AuthResponse = response.json().await.unwrap();

        let access_token = auth_data.access_token;

        let mut headers = header::HeaderMap::new();

        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
        );

        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&user_agent).unwrap(),
        );

        let client = Client::builder()
            .proxy(reqwest::Proxy::all(&http_proxy)?)
            .default_headers(headers)
            .build()
            .unwrap();
        error!("Successfully refreshed Reddit access token");
        Ok(client)
    }
}
