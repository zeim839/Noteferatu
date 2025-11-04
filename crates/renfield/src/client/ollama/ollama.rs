use std::time::Duration;
use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use serde_json::{Value, from_value};
use reqwest::header;

use super::request::Request;
use super::response::Response;
use super::model::{Model, ModelMetadata};
use super::error::Error as OllamaError;
use crate::Result;

/// Ollama API client.
pub struct Ollama {
    client: reqwest::Client,
    endpoint: Arc<String>,
}

impl Default for Ollama {

    /// Create a new client using the default local endpoint.
    fn default() -> Self {
        Self::new()
    }
}

impl Ollama {

    /// Create a new client using the default local endpoint.
    pub fn new() -> Self {
        let endpoint = "http://localhost:11434".to_string();
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(20))
            .build()
            .unwrap();

        Self { client, endpoint: Arc::new(endpoint) }
    }

    /// Create a new client that connects to a remote endpoint.
    ///
    /// Useful if connecting to remote machines or authenticating with
    /// Ollama private cloud.
    pub fn from_remote(endpoint: &str, api_key: Option<&str>) -> Self {
        let mut client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(20));

        if let Some(api_key) = api_key {
            let mut headers = header::HeaderMap::new();
            let mut auth = header::HeaderValue::from_str(&format!("Bearer {api_key}")).unwrap();
            auth.set_sensitive(true);
            headers.insert(header::AUTHORIZATION, auth);
            client = client.default_headers(headers);
        }

        let client = client.build().unwrap();
        Self { client, endpoint: Arc::new(endpoint.to_string()) }
    }

    /// Generate the next chat message in a conversation between a
    /// user and an assistant.
    pub async fn generate(&self, req: &Request) -> Result<Response> {
        let res = self.client.post(format!("{}/api/chat", self.endpoint))
            .json(&req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: String = from_value(error.clone())?;
            return Err(OllamaError::from(err).into());
        }

        Ok(from_value(json)?)
    }

    /// Stream the next chat message in a conversation between a user
    /// and an assistant.
    pub async fn stream(&self, req: &Request, pipe: Sender<Response>) -> Result<()> {
        todo!();
    }

    /// Fetch a list of models and their details.
    pub async fn list_models(&self) -> Result<Vec<Model>> {
        let res = self.client.get(format!("{}/api/tags", self.endpoint))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: String = from_value(error.clone())?;
            return Err(OllamaError::from(err).into());
        }

        Ok(from_value(json["models"].clone())?)
    }

    /// Fetch model details.
    pub async fn show_model_details(&self, model: &str) -> Result<ModelMetadata> {
        let res = self.client.post(format!("{}/api/show", self.endpoint))
            .json(&serde_json::json!({ "model": model }))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: String = from_value(error.clone())?;
            return Err(OllamaError::from(err).into());
        }

        Ok(from_value(json)?)
    }

    /// Retrieve the version of the Ollama.
    pub async fn get_version(&self) -> Result<String> {
        let res = self.client.get(format!("{}/api/version", self.endpoint))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: String = from_value(error.clone())?;
            return Err(OllamaError::from(err).into());
        }

        Ok(json["version"].as_str().unwrap_or_default().to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::client::ollama::*;
    use tokio::sync::OnceCell;

    static CLIENT: OnceCell<Ollama> = OnceCell::const_new();
    async fn get_test_client() -> &'static Ollama {
        CLIENT.get_or_init(|| async { Ollama::new() }).await
    }

    #[tokio::test]
    async fn test_api_error() {
        let client = get_test_client().await;
        let err = client.show_model_details("nonexistent model").await.unwrap_err();
        assert_eq!(err.to_string(), "invalid model path");
    }

    #[tokio::test]
    async fn test_generate() {
        todo!();
    }

    #[tokio::test]
    async fn test_generate_with_tools() {
        todo!();
    }

    #[tokio::test]
    async fn test_generate_with_reasoning() {
        todo!();
    }

    #[tokio::test]
    async fn test_stream() {
        todo!();
    }

    #[tokio::test]
    async fn test_list_models() {
        let client = get_test_client().await;
        let models = client.list_models().await.unwrap();
        assert!(models.len() > 0);
    }

    #[tokio::test]
    async fn test_show_model_details() {
        todo!();
    }

    #[tokio::test]
    async fn test_get_version() {
        let client = get_test_client().await;
        let version = client.get_version().await.unwrap();
        assert_ne!(version, "");
    }
}
