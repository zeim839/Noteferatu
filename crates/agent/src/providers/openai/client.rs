use super::response::Response;
use super::error::OpenAIError;
use super::request::Request;
use super::model::Model;

use crate::core::{Result, Error};
use crate::core;

use serde_json::{Value, from_value};
use tokio_stream::StreamExt;
use std::time::Duration;
use reqwest::header;

/// OpenAI API endpoint.
const API_ENDPOINT: &str = "https://api.openai.com/v1";

/// An OpenAI API client.
pub struct Client(reqwest::Client);

impl Client {

    /// Create a new OpenAI client.
    pub fn new(api_key: &str) -> Self {

        // Construct default request headers.
        let mut headers = header::HeaderMap::new();
        let auth = header::HeaderValue::from_str(&format!("Bearer {api_key}")).unwrap();
        headers.insert(header::AUTHORIZATION, auth);

        // Build HTTP client.
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(30))
            .read_timeout(Duration::from_secs(10))
            .default_headers(headers)
            .build().unwrap();

        Self(client)
    }

    /// Parses an OpenAI SSE event.
    pub(crate) fn parse_event(buffer: &mut String) -> Option<Response> {
        while let Some(double_newline_pos) = buffer.find("\n\n") {
            let event_block = buffer[..double_newline_pos].to_string();
            buffer.drain(..=double_newline_pos + 1);

            // Parse the event block line by line.
            for line in event_block.lines() {
                let line = line.trim();

                // Skip empty lines and non-data lines.
                if line.is_empty() || !line.starts_with("data: ") {
                    continue;
                }

                // Remove "data: " prefix.
                let data = &line[6..];

                // Check for stream termination.
                if data == "[DONE]" {
                    return None;
                }

                // Try to parse as JSON.
                match serde_json::from_str::<Response>(data) {
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
    type StreamResponse = Response;
    type Request = Request;
    type Response = Response;

    /// Send a non-streaming completion request to a selected model.
    ///
    /// API Reference: [Create Chat Completion](https://platform.openai.com/docs/api-reference/chat/create)
    async fn completion(&self, req: Request) -> Result<Response> {
        let req = req.with_stream(Some(false));
        let res = self.0.post(format!("{API_ENDPOINT}/chat/completions"))
            .json(&req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenAIError = from_value(error.clone())?;
            return Err(Error::OpenAI(err));
        }

        Ok(from_value(json)?)
    }

    /// Send a streaming completion request to a selected model.
    ///
    /// API Reference: [Stream Chat Completions](https://platform.openai.com/docs/api-reference/chat-streaming)
    async fn stream_completion<T>(&self, req: Request, mut cb: T) -> Result<()>
    where T: FnMut(Self::StreamResponse) {
        let req = req.with_stream(Some(true));
        let res = self.0.post(format!("{API_ENDPOINT}/chat/completions"))
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

        let err: OpenAIError = from_value(err.clone())?;
        return Err(Error::OpenAI(err));
    }

    /// List and describe the various models available in the API.
    ///
    /// API Reference: [Models](https://platform.openai.com/docs/api-reference/models)
    async fn list_models(&self) -> Result<Vec<core::Model>> {
        let res = self.0.get(format!("{API_ENDPOINT}/models"))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenAIError = from_value(error.clone())?;
            return Err(Error::OpenAI(err));
        }

        let models: Vec<Model> = from_value(json["data"].clone())?;
        Ok(models.into_iter().map(Into::into).collect())
    }

    /// Check if the client is connected.
    async fn check(&self) -> Result<()> {
        let res = self.0.get(format!("{API_ENDPOINT}/models"))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenAIError = from_value(error.clone())?;
            return Err(Error::OpenAI(err));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Client as _;
    use super::*;

    use std::env;
    use dotenv::dotenv;

    fn get_test_client() -> Client {
        dotenv().ok();
        let token = env::var("OPENAI_API_KEY")
            .expect("missing OPENAI_API_KEY");

        Client::new(&token)
    }

    #[tokio::test]
    async fn test_completion() {
        let client = get_test_client();
        let req = Request::from_prompt("gpt-4.1-mini", "hi")
            .with_max_completion_tokens(Some(5));

        let res = client.completion(req).await.unwrap();
        assert!(res.usage.as_ref().unwrap().completion_tokens <= 5);
        assert!(res.usage.as_ref().unwrap().completion_tokens > 0);
        assert!(res.choices.len() > 0);

        let msg = &res.choices[0].message.as_ref().unwrap();
        assert!(msg.content.as_ref().is_some());
    }

    #[tokio::test]
    async fn test_stream_completion() {
        let client = get_test_client();
        let req = Request::from_prompt("gpt-4.1-mini", "Hello")
            .with_max_completion_tokens(Some(5));

        let mut has_response = false;
        client.stream_completion(req, |_| { has_response = true; })
            .await.unwrap();

        assert!(has_response);
    }

    #[tokio::test]
    async fn test_list_models() {
        let client = get_test_client();
        let models = client.list_models().await.unwrap();
        assert!(models.len() > 0);
    }

    #[tokio::test]
    async fn test_check() {
        let check = get_test_client().check().await;
        assert!(check.is_ok());
        let check = Client::new("baddd-key").check().await;
        assert!(check.is_err());
    }
}
