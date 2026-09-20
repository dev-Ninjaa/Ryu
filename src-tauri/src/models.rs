use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValue {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthPayload {
    #[serde(default, rename = "type")]
    pub auth_type: String,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default, rename = "apiKey")]
    pub api_key: Option<String>,
    #[serde(default, rename = "apiValue")]
    pub api_value: Option<String>,
    #[serde(default, rename = "apiLocation")]
    pub api_location: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest {
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub params: Vec<KeyValue>,
    #[serde(default)]
    pub headers: Vec<KeyValue>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub auth: AuthPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: u16,
    #[serde(rename = "statusText")]
    pub status_text: String,
    pub time: u128,
    pub size: String,
    pub headers: HashMap<String, String>,
    pub body: serde_json::Value,
}
