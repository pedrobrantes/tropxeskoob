use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, ORIGIN, REFERER, USER_AGENT, AUTHORIZATION};
use anyhow::Result;
use crate::models::{BookshelfResponse, SkoobBook};
use tokio::time::{sleep, Duration};

pub struct SkoobClient {
    client: reqwest::Client,
    token: String,
    user_id: String,
    base_api_url: String,
}

impl SkoobClient {
    pub fn new(token: String, user_id: String) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Mobile Safari/537.36"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ORIGIN, HeaderValue::from_static("https://www.skoob.com.br"));
        headers.insert(REFERER, HeaderValue::from_static("https://www.skoob.com.br/"));
        
        let mut auth_val = HeaderValue::from_str(&token)?;
        auth_val.set_sensitive(true);
        headers.insert(AUTHORIZATION, auth_val);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            token,
            user_id,
            base_api_url: "https://prd-api.skoob.com.br/api/v1".to_string(),
        })
    }

    pub async fn fetch_bookshelf(&self, page: i32, limit: i32, bookshelf_type: &str, filter: &str) -> Result<BookshelfResponse> {
        let url = format!("{}/bookshelf", self.base_api_url);
        let params = [
            ("page", page.to_string()),
            ("limit", limit.to_string()),
            ("bookshelf_type", bookshelf_type.to_string()),
            ("user_id", self.user_id.clone()),
            ("filter", filter.to_string()),
            ("search_type", "title".to_string()),
        ];

        let response = self.client.get(&url)
            .query(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let data = response.json::<BookshelfResponse>().await?;
            Ok(data)
        } else {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("{} - {}", status, text)
        }
    }

    pub async fn fetch_all_books(&self, bookshelf_type: &str, filter: &str, sleep_time: f32) -> Result<Vec<SkoobBook>> {
        let mut all_books = Vec::new();
        let mut page = 1;

        loop {
            println!("[*] Requesting page {}...", page);
            let data = self.fetch_bookshelf(page, 30, bookshelf_type, filter).await?;
            let items = data.items;

            if items.is_empty() {
                break;
            }

            all_books.extend(items);
            let total_items = &data.total_items;
            println!("[+] Collected {}. Progress: {}/{}", all_books.len(), all_books.len(), total_items);

            if page >= data.total_pages {
                break;
            }

            page += 1;
            sleep(Duration::from_secs_f32(sleep_time)).await;
        }

        Ok(all_books)
    }
}
