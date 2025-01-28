#[cfg(not(target_arch = "wasm32"))]
use reqwest::Client;
#[cfg(target_arch = "wasm32")]
use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

pub struct HttpClient {
    #[cfg(not(target_arch = "wasm32"))]
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String, timeout: u64) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .expect("Failed to create HTTP client");

        #[cfg(target_arch = "wasm32")]
        {
            // В WASM версии timeout устанавливается через AbortController
            // Но на данный момент gloo_net не поддерживает эту функциональность
            // TODO: Реализовать timeout через AbortController когда будет доступно
        }

        Self { 
            #[cfg(not(target_arch = "wasm32"))]
            client,
            base_url 
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.base_url, path);
        
        #[cfg(target_arch = "wasm32")]
        {
            let response = Request::get(&url)
                .send()
                .await?
                .json()
                .await?;
            Ok(response)
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(self.client.get(&url).send().await?.json().await?)
        }
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.base_url, path);

        #[cfg(target_arch = "wasm32")]
        {
            let response = Request::post(&url)
                .json(body)?
                .send()
                .await?
                .json()
                .await?;
            Ok(response)
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(self.client.post(&url).json(body).send().await?.json().await?)
        }
    }
} 