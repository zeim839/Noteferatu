use super::stream::StreamResponse;
use super::error::AnthropicError;
use super::response::Response;
use super::request::Request;
use super::model::Model;

use crate::core::{Result, Error};
use crate::core;

use serde_json::{Value, from_value};
use tokio_stream::StreamExt;
use reqwest::header;

/// Anthropic API endpoint.
const API_ENDPOINT: &str = "https://api.anthropic.com/v1";

/// An Anthropic API client.
pub struct Client(reqwest::Client);

impl Client {

    /// Create a new Anthropic client.
    pub fn new(api_key: &str) -> Self {

        // Construct default request headers.
        let mut headers = header::HeaderMap::new();
        let api_key = header::HeaderValue::from_str(api_key).unwrap();
        let version = header::HeaderValue::from_str("2023-06-01").unwrap();

        headers.insert("x-api-key", api_key);
        headers.insert("anthropic-version", version);

        // Build HTTP client.
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build().unwrap();

        Self(client)
    }

    /// Parses an Anthropic SSE event.
    fn parse_event(buffer: &mut String) -> Option<StreamResponse> {
        // Look for complete SSE events (separated by double newlines)
        if let Some(event_end_pos) = buffer.find("\n\n") {
            let event_block = buffer[..event_end_pos].to_string();
            buffer.drain(..event_end_pos + 2); // +2 for \n\n

            // Parse the event block line by line
            for line in event_block.lines() {
                if !line.starts_with("data: ") {
                    continue;
                }

                // Remove "data: " prefix.
                let data = &line[6..];

                // Try to parse as JSON and return on first success
                if let Ok(response) = serde_json::from_str(data) {
                    return Some(response);
                }
            }
        }

        None
    }
}

impl core::Client for Client {

    type Error = core::Error;
    type StreamResponse = StreamResponse;
    type Request = Request;
    type Response = Response;

    /// Generate a non-streaming chat completion.
    ///
    /// API Reference: [Messages](https://docs.anthropic.com/en/api/messages)
    async fn completion(&self, req: Self::Request) -> Result<Response> {
        let req = req.with_stream(false);
        let res = self.0.post(format!("{API_ENDPOINT}/messages"))
            .json(&req).send().await?;

        let json: Value =  res.json().await?;
        if let Some(error) = json.get("error") {
            let err: AnthropicError = from_value(error.clone())?;
            return Err(Error::Anthropic(err));
        }

        Ok(from_value(json)?)
    }

    /// Generate a streaming chat completion.
    ///
    /// API Reference: [Messages](https://docs.anthropic.com/en/api/messages)
    async fn stream_completion<T>(&self, req: Self::Request, mut cb: T) -> Result<()>
    where T: FnMut(Self::StreamResponse) {
        let req = req.with_stream(true);
        let res = self.0.post(format!("{API_ENDPOINT}/messages"))
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
        let res = self.0.get(format!("{API_ENDPOINT}/models?limit=1000"))
            .send().await?.error_for_status()?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: AnthropicError = from_value(error.clone())?;
            return Err(core::Error::Anthropic(err));
        }

        let models: Vec<Model> = from_value(json["data"].clone())?;
        Ok(models.into_iter().map(Into::into).collect())
    }

    /// Check whether the client is connected.
    async fn check(&self) -> Result<()> {
        let res = self.0.get(format!("{API_ENDPOINT}/models?limit=1"))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: AnthropicError = from_value(error.clone())?;
            return Err(Error::Anthropic(err));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::ContentKind;
    use crate::core::Client as _;
    use super::*;

    use std::env;
    use dotenv::dotenv;
    const TEST_MODEL: &str = "claude-3-haiku-20240307";

    fn get_test_client() -> Client {
        dotenv().ok();
        let token = env::var("ANTHROPIC_API_KEY")
            .expect("missing ANTHROPIC_API_KEY env variable");

        Client::new(&token)
    }

    #[tokio::test]
    async fn test_completion() {
        let client = get_test_client();
        let req = Request::from_prompt(TEST_MODEL, "Hello")
            .with_max_tokens(5);

        let res = client.completion(req).await.unwrap();
        assert!(res.content.len() > 0);
        assert!(res.content[0].kind == ContentKind::Text);
        assert!(res.content[0].text.as_ref().unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_stream_completion() {
        let client = get_test_client();
        let req = Request::from_prompt(TEST_MODEL, "Hello")
            .with_max_tokens(5);

        let mut text = String::new();
        client.stream_completion(req, |res| {
            if let Some(delta) = res.delta {
                if let Some(t) = delta.text {
                    text.push_str(&t);
                }
            }
        }).await.unwrap();
        assert!(!text.is_empty());
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

        let client = Client::new("bad-api_token");
        assert!(client.check().await.is_err());
    }
}
