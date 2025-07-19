use crate::error::Error as E;
use super::models::Model;
use super::chat::*;
use crate::SSE;

use serde_json::{from_value, Value};

/// Alias for the Ollama error type.
pub type Error = E<String>;

/// Ollama API endpoint.
const API_ENDPOINT: &str = "http://localhost:11434/api";

/// An Ollama API client.
pub struct Client(reqwest::Client);

impl Client {

    /// Create a new Ollama API client.
    pub fn new() -> Self {
        Self(reqwest::Client::new())
    }

    /// Send a completion request to a selected model.
    pub async fn completion(&self, req: ChatRequest) -> Result<ChatResponse, Error> {
        let req = req.with_stream(Some(false));
        let res = self.0.post(format!("{API_ENDPOINT}/chat"))
            .json(&req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: String = from_value(error.clone())?;
            return Err(crate::Error::Api(err));
        }

        Ok(from_value(json)?)
    }

    /// Send a streaming completion request to a selected model.
    pub async fn stream_completion(&self, req: ChatRequest) -> Result<SSE<ChatResponse>, Error> {
        let req = req.with_stream(Some(true));
        let res = self.0.post(format!("{API_ENDPOINT}/chat"))
            .json(&req).send().await?;

        Ok(SSE::new(Box::new(Self::parse_event), res.bytes_stream()))
    }

    /// List available models.
    pub async fn list_models(&self) -> Result<Vec<Model>, Error> {
        let res = self.0.get(format!("{API_ENDPOINT}/tags"))
            .send().await?;

        let json: Value = res.json().await?;
        let mut models: Vec<Model> = from_value(json["models"].clone())?;
        for model in models.iter_mut() {
            let res = self.0.post(format!("{API_ENDPOINT}/show"))
                .json(&serde_json::json!({ "name": model.model.clone() }))
                .send().await;

            if let Ok(res) = res {
                if let Ok(json) = res.json::<Value>().await {
                    if let Some(map) = json.as_object() {
                        if let Some(model_info) = map.get("model_info").and_then(Value::as_object) {
                            // The context length is in a dynamic key, e.g. "llama.context_length".
                            for (key, value) in model_info.iter() {
                                if key.ends_with("context_length") {
                                    model.context_length = from_value(value.clone()).ok();
                                    break;
                                }
                            }
                        }
                        // Tool call capability is in the "capabilities" field.
                        if let Some(capabilities) = map.get("capabilities") {
                            if let Ok(caps) = from_value::<Vec<String>>(capabilities.clone()) {
                                model.supports_tool_calls = Some(caps.contains(&"tools".to_string()));
                            }
                        }
                    }
                }
            }
        }

        Ok(models)
    }

    /// Parses an Ollama SSE event.
    fn parse_event(buffer: &mut String) -> Option<ChatResponse> {
        while let Some(newline_pos) = buffer.find("\n") {
            let event_block = buffer[..newline_pos].to_string();
            buffer.drain(..=newline_pos);
            for line in event_block.lines() {
                match serde_json::from_str::<ChatResponse>(line.trim()) {
                    Ok(response) => return Some(response),
                    Err(_) => return None,
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openai::{FunctionDefinition, ToolDefinition};

    #[tokio::test]
    async fn test_completions() {
        let req = ChatRequest::from_prompt("gemma3n:e4b", "hi");
        let res = Client::new().completion(req).await.unwrap();
        assert!(res.message.content.is_some());
    }

    #[tokio::test]
    async fn test_stream_completion() {
        let req = ChatRequest::from_prompt("gemma3n:e4b", "hi");
        let mut stream = Client::new().stream_completion(req).await.unwrap();
        let mut response_count = 0;
        let mut has_text = false;
        while let Some(event) = stream.next::<String>().await {
            match event {
                Ok(data) => {
                    if data.message.content.is_some() {
                        has_text = true;
                    }
                    response_count += 1;
                    if has_text && response_count > 3 {
                        break;
                    }
                },
                Err(e) => panic!("stream error: {e}"),
            }
        }
        assert!(has_text);
        assert!(response_count > 0);
    }

    #[tokio::test]
    async fn test_list_models() {
        let models = Client::new().list_models().await.unwrap();
        assert!(models.len() > 0);
    }

    #[tokio::test]
    async fn test_tool_call() {
        let req = ChatRequest::from_prompt("qwen3:0.6b", "what's the current weather like?")
            .with_tools(Some(vec![ToolDefinition {
                kind: "function".to_string(),
                function: FunctionDefinition {
                    name: "get_current_weather".to_string(),
                    description: Some("retrieves the current weather".to_string()),
                    parameters: Some(
                        serde_json::from_str("{\"type\":\"object\",\"properties\":{}}")
                            .unwrap()
                    ),
                    strict: None,
                },
            }]));

        let res = Client::new().completion(req).await.unwrap();
        assert!(res.message.tool_calls.is_some());

        let tool_calls = res.message.tool_calls.as_ref().unwrap();
        assert!(!tool_calls.is_empty());
        assert_eq!(tool_calls[0].function.name, "get_current_weather");
    }
}
