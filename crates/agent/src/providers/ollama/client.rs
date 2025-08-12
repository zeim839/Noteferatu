use super::response::Response;
use super::request::Request;
use super::model::Model;

use crate::core::{Result, Error};
use crate::core;

use tokio_stream::StreamExt;
use std::time::Duration;
use serde_json::{Value, from_value};

/// An Ollama API client.
pub struct Client {
    client: reqwest::Client,
    endpoint: String,
}

impl Client {

    /// Create a new Ollama API client.
    pub fn new(endpoint: &str) -> Self {
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(30))
            // Higher timeout: models may take a while to load.
            .read_timeout(Duration::from_secs(120))
            .build().unwrap();

        let endpoint = format!("{endpoint}/api");
        Self { client, endpoint }
    }

    /// Parses an Ollama SSE event.
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
}

impl core::Client for Client {

    type Error = core::Error;
    type Request = Request;
    type Response = Response;
    type StreamResponse = Response;

    /// Send a completion request to a selected model.
    async fn completion(&self, req: Request) -> Result<Response> {
        let req = req.with_stream(Some(false));
        let res = self.client.post(format!("{}/chat", self.endpoint))
            .json(&req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: String = from_value(error.clone())?;
            return Err(Error::Ollama(err));
        }

        Ok(from_value(json)?)
    }

    /// Send a streaming completion request to a selected model.
    async fn stream_completion<T>(&self, req: Request, mut cb: T) -> Result<()>
    where T: FnMut(Response) {
        let req = req.with_stream(Some(true));
        let res = self.client.post(format!("{}/chat", self.endpoint))
            .json(&req).send().await?;

        if res.status().is_success() {
            let mut buffer = String::new();
            let mut stream = res.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                buffer.push_str(&String::from_utf8_lossy(&chunk));
                while let Some(event) = Self::parse_event(&mut buffer) {
                    cb(event);
                }
            }
            return Ok(());
        }

        let json: Value = res.json().await?;
        let err = json.get("error")
            .ok_or(Error::Json("expected \"error\" field".to_string()))?;

        let err: String = from_value(err.clone())?;
        return Err(Error::Ollama(err));
    }

    /// List available models.
    async fn list_models(&self) -> Result<Vec<core::Model>> {
        let res = self.client.get(format!("{}/tags", self.endpoint))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: String = from_value(error.clone())?;
            return Err(Error::Ollama(err));
        }

        let mut models: Vec<Model> = from_value(json["models"].clone())?;
        for model in models.iter_mut() {
            let res = self.client.post(format!("{}/show", self.endpoint))
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

        Ok(models.into_iter().map(Into::into).collect())
    }

    /// Check if the client is connected.
    async fn check(&self) -> Result<()> {
        let res = self.client.get(format!("{}/tags", self.endpoint))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: String = from_value(error.clone())?;
            return Err(Error::Ollama(err));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Client as _;
    use super::*;

    #[tokio::test]
    async fn test_completion() {
        let req = Request::from_prompt("gemma3n:e4b", "hi");
        let res = Client::new("http://localhost:11434").completion(req).await.unwrap();
        assert!(res.message.content.is_some());
    }

    #[tokio::test]
    async fn test_request_response() {
        let req = crate::core::Request {
            model: "gemma3n:e4b".to_string(),
            messages: vec![crate::core::Message {
                role: crate::core::Role::User,
                content: crate::core::MessageContent::Text("hi".to_string()),
            }],
            max_tokens: None,
            tools: vec![],
            system: None
        };

        let res = Client::new("http://localhost:11434")
            .completion(req.into())
            .await.unwrap();

        let res: crate::core::Response = res.into();
        let mut has_text_response = false;
        for message in res.messages {
            if let crate::core::MessageContent::Text(_) = message.content {
                has_text_response = true;
                break;
            }
        }

        assert!(has_text_response);
    }

    #[tokio::test]
    async fn test_stream_completion() {
        let req = Request::from_prompt("gemma3n:e4b", "hi");
        let client = Client::new("http://localhost:11434");
        let mut text = String::new();
        client.stream_completion(req, |res| {
            if let Some(content) = res.message.content {
                text += &content;
            }
        }).await.unwrap();
        assert!(text.len() > 0);
    }

    #[tokio::test]
    async fn test_stream_request_response() {
        let req = crate::core::Request {
            model: "gemma3n:e4b".to_string(),
            messages: vec![crate::core::Message {
                role: crate::core::Role::User,
                content: crate::core::MessageContent::Text("hi".to_string()),
            }],
            max_tokens: None,
            tools: vec![],
            system: None
        };

        let mut has_text_response = false;
        Client::new("http://localhost:11434")
            .stream_completion(req.into(), |res| {
                let res: crate::core::Response = res.into();
                for message in res.messages {
                    if let crate::core::MessageContent::Text(_) = message.content {
                        has_text_response = true;
                        break;
                    }
                }
            }).await.unwrap();

        assert!(has_text_response);
    }

    #[tokio::test]
    async fn test_list_models() {
        let models = Client::new("http://localhost:11434").list_models().await.unwrap();
        assert!(models.len() > 0);
    }

    #[tokio::test]
    async fn test_check() {
        let check = Client::new("http://localhost:11434")
            .check().await;

        assert!(check.is_ok());
        let check = Client::new("bad-url").check().await;
        assert!(check.is_err())
    }
}
