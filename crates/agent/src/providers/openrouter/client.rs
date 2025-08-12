use crate::providers::openai::{Response, Client as OAI};
use super::error::OpenRouterError;
use crate::core::{Result, Error};
use super::request::Request;
use super::model::Model;
use crate::core;

use serde_json::{Value, from_value};
use tokio_stream::StreamExt;
use std::time::Duration;
use reqwest::header;

/// OpenRouter API endpoint.
const API_ENDPOINT: &str = "https://openrouter.ai/api/v1";

/// An OpenRouter API client.
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
}

impl core::Client for Client {

    type Error = core::Error;
    type StreamResponse = Response;
    type Request = Request;
    type Response = Response;

    /// Send a chat completion request to a selected model.
    ///
    /// API Reference: [Chat Completion](https://openrouter.ai/docs/api-reference/chat-completion)
    async fn completion(&self, req: Request) -> Result<Response> {
        let req = req.with_stream(Some(false));
        let res = self.0.post(format!("{API_ENDPOINT}/chat/completions"))
            .json(&req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenRouterError = from_value(error.clone())?;
            return Err(Error::OpenRouter(err));
        }

        Ok(from_value(json)?)
    }

    /// Send a streaming chat completion request to a selected model.
    ///
    /// API Reference: [Chat Completion](https://openrouter.ai/docs/api-reference/chat-completion)
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
                while let Some(event) = OAI::parse_event(&mut buffer) {
                    cb(event);
                }
            }
            return Ok(());
        }

        let json: Value = res.json().await?;
        let err = json.get("error")
            .ok_or(Error::Json("expected \"error\" field".to_string()))?;

        let err: OpenRouterError = from_value(err.clone())?;
        return Err(Error::OpenRouter(err));
    }

    /// Fetches a list of models available via the API.
    ///
    /// API Reference: [List Available Models](https://openrouter.ai/docs/api-reference/list-available-models)
    async fn list_models(&self) -> Result<Vec<core::Model>> {
        let res = self.0.get(format!("{API_ENDPOINT}/models"))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenRouterError = from_value(error.clone())?;
            return Err(Error::OpenRouter(err));
        }

        let models: Vec<Model> = from_value(json["data"].clone())?;
        Ok(models.into_iter().map(Into::into).collect())
    }

    /// Check if the client is connected.
    async fn check(&self) -> Result<()> {
        let res = self.0.post(format!("{API_ENDPOINT}/completions"))
            .json(&serde_json::json!({
                "model": "mistralai/devstral-small-2505:free",
                "models": vec![
                    "google/gemma-3n-e2b-it:free",
                    "mistralai/mistral-small-3.2-24b-instruct:free",
                    "deepseek/deepseek-r1-0528:free",
                ],
                "prompt": "hi",
                "max_tokens": 1,
            }))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenRouterError = from_value(error.clone())?;
            return Err(Error::OpenRouter(err));
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

    const TEST_MODEL: &str = "deepseek/deepseek-chat-v3-0324:free";

    fn get_test_client() -> Client {
        dotenv().ok();
        let token = env::var("OPENROUTER_API_KEY")
            .expect("missing OPENROUTER_API_KEY env variable");

        Client::new(&token)
    }

    #[tokio::test]
    async fn test_completion() {
        let client = get_test_client();
        let req = Request::from_prompt(TEST_MODEL, "hi")
            .with_max_tokens(Some(5));

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
        let req = Request::from_prompt(TEST_MODEL, "Hello")
            .with_max_tokens(Some(5));

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
        let check = Client::new("badd-key").check().await;
        assert!(check.is_err());
    }
}
