use super::chat::{ChatRequest, ChatResponse};
use super::errors::{ClientError, OllamaError};
use super::stream::StreamSSE;
use super::models::Model;

#[allow(unused)]
use tokio_stream::StreamExt;
use serde_json::{from_value, Value};

/// Ollama API endpoint.
const API_ENDPOINT: &str = "http://localhost:11434/api";

/// An Ollama API client.
pub struct Client(reqwest::Client);

impl Client {

    /// Create a new Ollama API client.
    pub fn new() -> Self {
        Self(reqwest::Client::new())
    }

    /// Send a completion request to a selected model. For streaming
    /// responses, see [Client::stream_completion].
    pub async fn completions(&self, req: ChatRequest) -> Result<ChatResponse, ClientError> {
        let req = req.with_stream(Some(false));
        let res = self.0.post(format!("{API_ENDPOINT}/chat"))
            .json(&req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OllamaError = from_value(error.clone())?;
            return Err(ClientError::Api(err));
        }

        Ok(from_value(json)?)
    }

    /// Send a completion request to a selected model, returning a SSE
    /// stream. For non-streaming completions, see [Client::completion].
    pub async fn stream_completion(&self, req: ChatRequest) -> Result<StreamSSE, ClientError> {
        let req = req.with_stream(Some(true));
        let res = self.0.post(format!("{API_ENDPOINT}/chat"))
            .json(&req).send().await?;

        Ok(StreamSSE::new(res.bytes_stream()))
    }

    /// List available models.
    pub async fn list_models(&self) -> Result<Vec<Model>, ClientError> {
        let res = self.0.get(format!("{API_ENDPOINT}/tags"))
            .send().await?;

        let json: Value = res.json().await?;
        Ok(from_value(json["models"].clone())?)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_completions() {
        let req = ChatRequest::from_prompt("gemma3n:e4b", "hi");
        let res = Client::new().completions(req).await.unwrap();
        assert!(res.message.content.is_some());
    }

    #[tokio::test]
    async fn test_stream_completion() {
        let req = ChatRequest::from_prompt("gemma3n:e4b", "hi");
        let mut stream = Client::new().stream_completion(req).await.unwrap();
        let mut response_count = 0;
        while let Some(result) = stream.next().await {
            match result {
                Ok(_) => {
                    response_count += 1;
                    if response_count >= 3 {
                        break;
                    }
                },
                Err(e) => panic!("stream error: {e}"),
            }
        }
        assert!(response_count > 0, "Should receive at least one response");
    }

    #[tokio::test]
    async fn test_list_models() {
        let models = Client::new().list_models().await.unwrap();
        assert!(models.len() > 0);
    }
}
