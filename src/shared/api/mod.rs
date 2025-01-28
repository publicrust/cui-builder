// API interfaces
pub mod http;
pub mod websocket;

pub struct ApiConfig {
    pub base_url: String,
    pub timeout: u64,
} 