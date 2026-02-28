use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkoobBook {
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct BookshelfResponse {
    pub items: Vec<SkoobBook>,
    pub total_items: serde_json::Value, // Sometimes string, sometimes int
    pub total_pages: i32,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub response: Option<LoginResponseData>,
    pub token: Option<String>,
    pub user: Option<UserInfo>,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponseData {
    pub token: Option<String>,
    pub user: Option<UserInfo>,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub id: serde_json::Value,
}
