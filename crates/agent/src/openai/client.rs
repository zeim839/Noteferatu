use super::chat::{ChatRequest, ChatResponse};
use super::errors::{ClientError, OpenAIError};
use super::stream::StreamSSE;
use super::models::Model;

#[allow(unused)]
use tokio_stream::StreamExt;

use reqwest::header;
use serde_json::{Value, from_value};

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
            .default_headers(headers)
            .build().unwrap();

        Self(client)
    }

    /// Send a completion request to a selected model. For streaming
    /// responses, see [Client::stream_completion].
    ///
    /// API Reference: [Create Chat Completion](https://platform.openai.com/docs/api-reference/chat/create)
    pub async fn completion(&self, req: ChatRequest) -> Result<ChatResponse, ClientError> {
        let req = req.with_stream(Some(false));
        let res = self.0.post(format!("{API_ENDPOINT}/chat/completions"))
            .json(&req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenAIError = from_value(error.clone())?;
            return Err(ClientError::Api(err));
        }

        Ok(from_value(json)?)
    }

    /// Send a completion request to a selected model, returning a SSE
    /// stream. For non-streaming completions, see [Client::completion].
    ///
    /// API Reference: [Stream Chat Completions](https://platform.openai.com/docs/api-reference/chat-streaming)
    pub async fn stream_completion(&self, req: ChatRequest) -> Result<StreamSSE, ClientError> {
        let req = req.with_stream(Some(true));
        let res = self.0.post(format!("{API_ENDPOINT}/chat/completions"))
            .json(&req).send().await?;

        Ok(StreamSSE::new(res.bytes_stream()))
    }

    /// List and describe the various models available in the API.
    ///
    /// API Reference: [Models](https://platform.openai.com/docs/api-reference/models)
    pub async fn list_models(&self) -> Result<Vec<Model>, ClientError> {
        let res = self.0.get(format!("{API_ENDPOINT}/models"))
            .send().await?;

        let json: Value = res.json().await?;
        Ok(from_value(json["data"].clone())?)
    }
}

#[cfg(test)]
mod tests {
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
        let req = ChatRequest::from_prompt(
            "gpt-4.1-mini", "hi"
        ).with_max_completion_tokens(Some(5));

        let res = client.completion(req).await.unwrap();
        assert!(res.usage.as_ref().unwrap().completion_tokens <= 5);
        assert!(res.usage.as_ref().unwrap().completion_tokens > 0);
        assert!(res.choices.len() > 0);

        let msg = &res.choices[0].message.as_ref().unwrap();
        assert!(msg.content.as_ref().unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_stream_completion() {
        let client = get_test_client();
        let req = ChatRequest::from_prompt(
            "gpt-4.1-mini", "hi"
        ).with_max_completion_tokens(Some(5));

        let mut stream = client.stream_completion(req).await.unwrap();
        let mut response_count = 0;
        while let Some(result) = stream.next().await {
            match result {
                Ok(_) => {
                    response_count += 1;

                    // Limit test to avoid long execution
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
        let client = get_test_client();
        let models = client.list_models().await.unwrap();
        assert!(models.len() > 0);
    }
}
