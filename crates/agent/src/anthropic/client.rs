use super::errors::{ClientError, AnthropicError};
use super::stream::StreamSSE;
use super::models::Model;
use super::messages::*;

#[allow(unused)]
use tokio_stream::StreamExt;

use reqwest::header;
use serde_json::{Value, from_value};

/// Anthropic API endpoint.
const API_ENDPOINT: &str = "https://api.anthropic.com/v1";

/// An Anthropic API client.
pub struct Client(reqwest::Client);

impl Client {

    /// Create a new Anthropic API client.
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

    /// Send a message request.
    ///
    /// API Reference: [Messages](https://docs.anthropic.com/en/api/messages)
    pub async fn messages(&self, req: MessageRequest) -> Result<MessageResponse, ClientError> {
        let req = req.with_stream(false);
        let res = self.0.post(format!("{API_ENDPOINT}/messages"))
            .json(&req)
            .send().await
            .map_err(|e| ClientError::from(e))?;

        let json: Value =  res.json()
            .await.map_err(|e| ClientError::from(e))?;

        if let Some(error) = json.get("error") {
            let err: AnthropicError = from_value(error.clone())
                .map_err(|e| ClientError::from(e))?;

            return Err(ClientError::Api(err));
        }

        let chat_res: MessageResponse = from_value(json)
            .map_err(|e| ClientError::from(e))?;

        Ok(chat_res)
    }

    /// Send a message request and stream the response.
    ///
    /// API Reference: [Messages](https://docs.anthropic.com/en/api/messages)
    pub async fn stream_messages(&self, req: MessageRequest) -> Result<StreamSSE, ClientError> {
        let req = req.with_stream(true);
        let res = self.0.post(format!("{API_ENDPOINT}/messages"))
            .json(&req)
            .send().await
            .map_err(|e| ClientError::from(e))?;

        let stream = res.bytes_stream();
        Ok(StreamSSE::new(stream))
    }

    /// Fetches a list of models available through the API.
    ///
    /// API Reference: [List Models](https://docs.anthropic.com/en/api/models-list)
    pub async fn list_models(&self) -> Result<Vec<Model>, ClientError> {
        let res = self.0.get(format!("{API_ENDPOINT}/models?limit=1000"))
            .send().await
            .map_err(|e| ClientError::from(e))?;

        let json: Value = res.json()
            .await.map_err(|e| ClientError::from(e))?;

        let models: Vec<Model> = from_value(json["data"].clone())
            .map_err(|e| ClientError::from(e))?;

        Ok(models)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env;
    use dotenv::dotenv;

    fn get_test_client() -> Client {
        dotenv().ok();
        let token = env::var("ANTHROPIC_API_KEY")
            .expect("missing ANTHROPIC_API_KEY env variable");

        Client::new(&token)
    }

    #[tokio::test]
    async fn test_messages() {
        let client = get_test_client();
        let req = MessageRequest::from_prompt(
            "claude-3-haiku-20240307", "Hello"
        ).with_max_tokens(5);

        let res = client.messages(req).await.unwrap();
        assert!(res.content.len() > 0);

        let content = &res.content[0];
        if let ContentResponse::Text(text) = content {
            assert!(text.text.len() > 0);
            return;
        }

        panic!("did not receive text response");
    }

    #[tokio::test]
    async fn test_stream_messages() {
        let client = get_test_client();
        let req = MessageRequest::from_prompt(
            "claude-3-haiku-20240307", "Hello"
        ).with_max_tokens(5);

        let mut stream = client.stream_messages(req).await.unwrap();
        let mut response_count = 0;

        while let Some(result) = stream.next().await {
            match result {
                Ok(_) => {
                    response_count += 1;

                    // Limit test to avoid long execution.
                    if response_count >= 5 {
                        break;
                    }
                },
                Err(e) => panic!("stream error: {e}"),
            }
        }
    }

    #[tokio::test]
    async fn test_list_models() {
        let client = get_test_client();
        let models = client.list_models().await.unwrap();
        assert!(models.len() > 0);
    }
}
