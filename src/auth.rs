use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, ORIGIN, REFERER, USER_AGENT};
use serde_json::json;
use anyhow::Result;
use crate::skoob::models::LoginResponse;

pub struct SkoobAuth {
    client: reqwest::Client,
    base_api_url: String,
    // base_url: String, // unused for now but present in original
}

impl SkoobAuth {
    pub fn new() -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Mobile Safari/537.36"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ORIGIN, HeaderValue::from_static("https://www.skoob.com.br"));
        headers.insert(REFERER, HeaderValue::from_static("https://www.skoob.com.br/"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            base_api_url: "https://prd-api.skoob.com.br/api/v1".to_string(),
            // base_url: "https://www.skoob.com.br".to_string(),
        })
    }

    pub async fn signin(&self, email: &str, password: &str) -> Result<LoginResponse> {
        let url = format!("{}/user/signin-by-password", self.base_api_url);
        let payload = json!({
            "email": email,
            "password": password
        });

        let response = self.client.post(&url)
            .header("platform-origin-login", "web")
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let data = response.json::<LoginResponse>().await?;
            Ok(data)
        } else {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("{} - {}", status, text)
        }
    }
}
