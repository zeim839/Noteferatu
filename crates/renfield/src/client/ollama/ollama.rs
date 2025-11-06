use std::time::Duration;
use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use serde_json::{Value, from_value};
use reqwest::header;
use tokio_stream::StreamExt;

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
            .json(req).send().await?;

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
        let res = self.client.post(format!("{}/api/chat", self.endpoint))
            .json(req).send().await?;

        if res.status().is_success() {
            let mut buffer = String::new();
            let mut stream = res.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                buffer.push_str(&String::from_utf8_lossy(&chunk));
                while let Some(event) = Self::parse_event(&mut buffer) {
                    // Pipe probably closed.
                    if let Err(_) = pipe.send(event).await {
                        return Ok(());
                    }
                }
            }
            return Ok(());
        }

        let json: Value = res.json().await?;
        let err = json.get("error").unwrap_or_default();
        let err: String = from_value(err.clone())?;
        return Err(OllamaError::from(err).into());
    }

    /// Parses an Ollama SSE event.
    #[inline]
    fn parse_event(buffer: &mut String) -> Option<Response> {
        while let Some(newline_pos) = buffer.find("\n") {
            let event_block = buffer[..newline_pos].to_string();
            buffer.drain(..=newline_pos);
            for line in event_block.lines() {
                match serde_json::from_str::<Response>(line.trim()) {
                    Ok(response) => return Some(response),
                    Err(_) => return None,
                }
            }
        }
        None
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
    use crate as renfield;
    use crate::client::ollama::*;
    use crate::tools::{Tool, tool, schema};
    use once_cell::sync::Lazy;

    static CLIENT: Lazy<Ollama> = Lazy::new(|| {
        Ollama::new()
    });

    #[tokio_shared_rt::test(shared)]
    async fn test_api_error() {
        let err = CLIENT.show_model_details("nonexistent model").await.unwrap_err();
        assert_eq!(err.to_string(), "invalid model path");
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate() {
        let req = Request::from_model("qwen3:0.6b")
            .push_message(msg!("user", "hello, who are you?"))
            .num_predict(10)
            .think(false);

        let res = CLIENT.generate(&req).await.unwrap();
        if let Message::Assistant { content, .. } = res.message {
            assert_ne!(content.len(), 0);
            return;
        }

        panic!("model did not respond with content");
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_with_tools() {

        // Ignore "unused fields" warnings.
        #![allow(dead_code)]

        /// Get the current weather at a specific location.
        #[derive(tool, schema)]
        #[tool(rename = "get_weather")]
        struct GetWeather {

            /// City name, followed by country (e.g. Bogota, Columbia)
            location: String,
        }

        let req = Request::from_model("qwen3:0.6b")
            .push_message(msg!("user", "what is the current weather in Paris, France?"))
            .num_predict(250)
            .push_tool(GetWeather::as_ollama_tool())
            .think(true);

        let res = CLIENT.generate(&req).await.unwrap();
        if let Message::Assistant { tool_calls, .. } = res.message {
            assert!(tool_calls.is_some());
            let tool_calls = tool_calls.unwrap();
            let mut has_call = false;
            for tool_call in tool_calls {
                let func = tool_call.function;
                // Don't care about arguments.
                // Model is shit anyway.
                if func.name == "get_weather" {
                    has_call = true;
                    break;
                }
            }
            assert!(has_call);
            return;
        }

        panic!("model did not respond with tool call");
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_with_reasoning() {
        let req = Request::from_model("qwen3:0.6b")
            .push_message(msg!("user", "what is 5*5+25? Think real hard"))
            .num_predict(100)
            .think(true);

        let res = CLIENT.generate(&req).await.unwrap();
        if let Message::Assistant { thinking, content, .. } = res.message {
            if let Some(thinking) = thinking {
                assert_ne!(thinking.len(), 0);
                return;
            }
            // The model is stupid... sometimes it "thinks" in content.
            assert!(content.contains("<think>"));
            return;
        }

        panic!("model did not respond with reasoning");
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_stream() {
        let req = Request::from_model("qwen3:0.6b")
            .push_message(msg!("user", "hello, who are you?"))
            .num_predict(20)
            .stream(true)
            .think(false);

        let (tx, mut rx) = tokio::sync::mpsc::channel::<Response>(4);
        let handler = tokio::spawn(async move {
            CLIENT.stream(&req, tx).await.unwrap();
        });

        let mut eval_tokens = 0;
        while let Some(res) = rx.recv().await {
            if res.done {
                assert!(res.eval_count.is_some());
                eval_tokens = res.eval_count.unwrap();
            }
        }

        assert!(handler.await.is_ok());
        assert!(eval_tokens > 0);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_list_models() {
        let models = CLIENT.list_models().await.unwrap();
        assert!(models.len() > 0);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_show_model_details() {
        let details = CLIENT.show_model_details("qwen3:0.6b").await.unwrap();
        assert!(details.parameters.len() > 0);
        assert!(details.capabilities.len() > 0);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_get_version() {
        let version = CLIENT.get_version().await.unwrap();
        assert_ne!(version, "");
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_cloud() {
        dotenv::dotenv().ok();
        let api_key = std::env::var("OLLAMA_API_KEY")
            .expect("missing Ollama cloud API key");

        let client = Ollama::from_remote("https://ollama.com", Some(&api_key));
        let req = Request::from_model("gpt-oss:20b-cloud")
            .push_message(msg!("user", "hello, who are you?"))
            .num_predict(10)
            .think(false);

        let res = client.generate(&req).await.unwrap();
        if let Message::Assistant { content, thinking, .. } = res.message {
            assert!(content.len() != 0 || thinking.is_some());
            return;
        }

        panic!("model did not respond with text");
    }
}
