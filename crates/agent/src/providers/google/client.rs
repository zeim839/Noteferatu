use super::response::Response;
use super::error::GoogleError;
use super::request::Request;
use super::model::Model;

use crate::core::{Result, Error};
use crate::core;

use serde_json::{Value, from_value};
use tokio_stream::StreamExt;
use reqwest::header;

/// Google Gemini API endpoint.
const API_ENDPOINT: &str = "https://generativelanguage.googleapis.com/v1beta";

/// A Google Gemini API client.
pub struct Client(reqwest::Client);

impl Client {

    /// Create a new Google Gemini client.
    pub fn new(api_key: &str) -> Self {

        // Construct default request headers.
        let mut headers = header::HeaderMap::new();
        let auth = header::HeaderValue::from_str(api_key).unwrap();
        headers.insert("x-goog-api-key", auth);

        // Build HTTP client.
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build().unwrap();

        Self(client)
    }

    /// Parses a Gemini SSE event.
    fn parse_event(buffer: &mut String) -> Option<Response> {
        let mut start_idx = 0;
        let mut brace_count = 0;
        let mut object_start = None;

        // Skip leading whitespace and array opening bracket
        while start_idx < buffer.len() && (buffer.as_bytes()[start_idx] as char).is_ascii_whitespace() {
            start_idx += 1;
        }
        if start_idx < buffer.len() && buffer.as_bytes()[start_idx] == b'[' {
            start_idx += 1;
        }

        // Find the start of a JSON object '{'
        for i in start_idx..buffer.len() {
            match buffer.as_bytes()[i] {
                b'{' => {
                    if object_start.is_none() {
                        object_start = Some(i);
                    }
                    brace_count += 1;
                },
                b'}' => {
                    brace_count -= 1;
                    if brace_count == 0 && object_start.is_some() {
                        // Found a complete JSON object
                        let obj_start = object_start.unwrap();
                        let obj_end = i + 1; // Inclusive '}'
                        let json_str_to_parse = &buffer[obj_start..obj_end];
                        match serde_json::from_str::<Response>(json_str_to_parse) {
                            Ok(response) => {
                                // After parsing, remove the parsed object and any trailing comma or array closing bracket
                                let mut drain_end = obj_end;
                                // Check for comma or ']' after the object
                                while drain_end < buffer.len() && (buffer.as_bytes()[drain_end] as char).is_ascii_whitespace() {
                                    drain_end += 1;
                                }
                                if drain_end < buffer.len() && buffer.as_bytes()[drain_end] == b',' {
                                    drain_end += 1;
                                } else if drain_end < buffer.len() && buffer.as_bytes()[drain_end] == b']' {
                                    // This ']' might be the final one, or intermediate if multiple are closing
                                    // Let's not consume it prematurely unless it's truly the end of the stream.
                                    // For now, if we found a valid object, just consume up to the object + any following comma
                                    // The end of stream will be handled in poll_next when `self.inner` returns None.
                                }
                                buffer.drain(..drain_end);
                                return Some(response);
                            },
                            Err(_) => {
                                buffer.clear();
                                return None;
                            }
                        }
                    }
                },
                // Handle potential array start/end or other non-JSON object chars
                b'[' | b']' | b',' => {
                    // Ignore these outside of brace counting, as they are array delimiters/separators.
                    // If we are at the very beginning and see '[', we should advance start_idx.
                }
                _ => {} // Other characters are part of the JSON content
            }
        }
        None
    }
}

impl core::Client for Client {

    type Error = core::Error;
    type StreamResponse = Response;
    type Request = Request;
    type Response = Response;

    /// Generate a non-streaming chat completion.
    ///
    /// API Reference: [models.generateContent](https://ai.google.dev/api/generate-content#method:-models.generatecontent)
    async fn completion(&self, req: Self::Request) -> Result<Response> {
        let model = &req.model;
        let res = self.0.post(format!("{API_ENDPOINT}/models/{model}:generateContent"))
            .json(&req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: GoogleError = from_value(error.clone())?;
            return Err(Error::Google(err));
        }

        Ok(from_value(json)?)
    }

    /// Generate a streaming chat completion.
    ///
    /// API Reference: [models.streamGenerateContent](https://ai.google.dev/api/generate-content#method:-models.streamgeneratecontent)
    async fn stream_completion<T>(&self, req: Self::Request, mut cb: T) -> Result<()>
    where T: FnMut(Self::StreamResponse) {
        let model = &req.model;
        let res = self.0.post(format!("{API_ENDPOINT}/models/{model}:streamGenerateContent"))
            .json(&req).send().await?;

        let mut buffer = String::new();
        let mut stream = res.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            buffer.push_str(&String::from_utf8_lossy(&chunk));
            while let Some(event) = Self::parse_event(&mut buffer) {
                cb(event);
            }
        }

        Ok(())
    }

    /// List the models available on the client.
    ///
    /// API Reference: [List Models](https://docs.anthropic.com/en/api/models-list)
    async fn list_models(&self) -> Result<Vec<core::Model>> {
        let res = self.0.get(format!("{API_ENDPOINT}/models?pageSize=1000"))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: GoogleError = from_value(error.clone())?;
            return Err(Error::Google(err));
        }

        let models: Vec<Model> = from_value(json["models"].clone())?;
        Ok(models.into_iter().map(Into::into).collect())
    }

    /// Check whether the client is connected.
    async fn check(&self) -> Result<()> {
        let res = self.0.get(format!("{API_ENDPOINT}/models?pageSize=1"))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: GoogleError = from_value(error.clone())?;
            return Err(Error::Google(err));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::core::Client as _;

    use std::env;
    use dotenv::dotenv;

    fn get_test_client() -> Client {
        dotenv().ok();
        let token = env::var("GOOGLE_API_KEY")
            .expect("missing GOOGLE_API_KEY env variable");

        Client::new(&token)
    }

    #[tokio::test]
    async fn test_completion() {
        let client = get_test_client();
        let config = GenerationConfig::new()
            .with_max_output_tokens(Some(10))
            .with_thinking_config(Some(ThinkingConfig{
                include_thoughts: true,
                thinking_budget: 128,
            }));

        let req = Request::from_prompt("gemini-2.5-pro", "hi")
            .with_generation_config(Some(config));

        let res = client.completion(req).await.unwrap();
        assert!(res.candidates.len() > 0);

        let candidate = &res.candidates[0];
        if let Some(txt) = &candidate.content.parts[0].data.text {
            assert!(txt.len() > 0);
        } else {
            panic!("response did not include text");
        }

        let usage_metadata = res.usage_metadata.unwrap();
        assert!(usage_metadata.total_token_count.unwrap() > 0);
    }

    const TEST_TOOL: &str = "{\"type\": \"object\", \"properties\":{}}";

    #[tokio::test]
    async fn test_request_response() {
        let client = get_test_client();
        let req = crate::core::Request {
            model: "gemini-2.5-pro".to_string(),
            messages: vec![crate::core::Message {
                role: crate::core::Role::User,
                content: crate::core::MessageContent::Text("what's the current weather?".to_string()),
            }],
            max_tokens: None,
            tools: vec![crate::core::ToolDefinition{
                name: "current_weather".to_string(),
                description: "fetch the latest local weather".to_string(),
                parameters: Some(serde_json::from_str(TEST_TOOL).unwrap()),
            }],
            system: Some("answer the user's query".to_string())
        };

        let res = client.completion(req.into()).await.unwrap();
        let res: crate::core::Response = res.into();
        assert!(res.messages.len() > 0);

        let mut has_tool_call = false;
        let mut has_text_response = false;
        for message in res.messages {
            if let crate::core::MessageContent::ToolCall(_) = message.content {
                has_tool_call = true;
            }
            if let crate::core::MessageContent::Text(_) = message.content {
                has_text_response = true;
            }
        }

        assert!(has_tool_call);
        assert!(has_text_response);
    }

    #[tokio::test]
    async fn test_stream_completion() {
        let client = get_test_client();
        let config = GenerationConfig::new()
            .with_max_output_tokens(Some(10))
            .with_thinking_config(Some(ThinkingConfig{
                include_thoughts: true,
                thinking_budget: 128,
            }));

        let req = Request::from_prompt("gemini-2.5-pro", "hi")
            .with_generation_config(Some(config));

        let mut has_text = false;
        client.stream_completion(req, |res| {
            for candidate in res.candidates {
                for part in candidate.content.parts {
                    if let Some(_) = part.data.text {
                        has_text = true;
                    }
                }
            }
        }).await.unwrap();
        assert!(has_text);
    }

    #[tokio::test]
    async fn test_stream_request_response() {
        let client = get_test_client();
        let req = crate::core::Request {
            model: "gemini-2.5-pro".to_string(),
            messages: vec![crate::core::Message {
                role: crate::core::Role::User,
                content: crate::core::MessageContent::Text("what's the current weather?".to_string()),
            }],
            max_tokens: None,
            tools: vec![crate::core::ToolDefinition{
                name: "current_weather".to_string(),
                description: "fetch the latest local weather".to_string(),
                parameters: Some(serde_json::from_str(TEST_TOOL).unwrap()),
            }],
            system: Some("answer the user's query".to_string())
        };

        let mut has_tool_call = false;
        let mut has_text_response = false;
        client.stream_completion(req.into(), |res| {
            let res: crate::core::Response = res.into();
            for message in res.messages {
                if let crate::core::MessageContent::ToolCall(_) = message.content {
                    has_tool_call = true;
                }
                if let crate::core::MessageContent::Text(_) = message.content {
                    has_text_response = true;
                }
            }
        }).await.unwrap();

        assert!(has_tool_call);
        assert!(has_text_response);
    }

    #[tokio::test]
    async fn test_list_models() {
        let client = get_test_client();
        let models = client.list_models().await.unwrap();
        assert!(models.len() > 0);
    }

    #[tokio::test]
    async fn test_check() {
        let client = get_test_client();
        assert!(client.check().await.is_ok());

        let client = Client::new("bad-api-token");
        assert!(client.check().await.is_err());
    }
}
