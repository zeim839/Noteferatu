use super::errors::{ClientError, OpenRouterError};
use super::models::Model;
use super::chat::{ChatRequest, ChatResponse};
use super::stream::StreamSSE;

#[allow(unused)]
use tokio_stream::StreamExt;

use reqwest::header;
use serde_json::{Value, from_value};

/// OpenRouter API endpoint.
const API_ENDPOINT: &str = "https://openrouter.ai/api/v1";

/// An OpenRouter API client.
pub struct Client(reqwest::Client);

impl Client {

    /// Create a new OpenRouter client.
    pub fn new(api_key: &str) -> Self {

        // Construct default request headers.
        let mut headers = header::HeaderMap::new();
        let auth = header::HeaderValue::from_str(&format!("Bearer {api_key}")).unwrap();
        let referer = header::HeaderValue::from_str("https://noteferatu.com").unwrap();
        let xtitle = header::HeaderValue::from_str("NoteFeratu").unwrap();

        headers.insert(header::AUTHORIZATION, auth);
        headers.insert("HTTP-Referer", referer);
        headers.insert("X-Title", xtitle);

        // Build HTTP client.
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build().unwrap();

        Self(client)
    }

    /// Send a completion request to a selected model. For streaming
    /// responses, see [Client::stream_completion].
    ///
    /// API Reference: [Completion](https://openrouter.ai/docs/api-reference/completion)
    pub async fn completion(&self, req: ChatRequest) -> Result<ChatResponse, ClientError> {
        let req = req.with_stream(Some(false));
        let res = self.0.post(format!("{API_ENDPOINT}/completions"))
            .json(&req)
            .send().await
            .map_err(|e| ClientError::from(e))?;

        let json: Value = res.json()
            .await.map_err(|e| ClientError::from(e))?;

        if let Some(error) = json.get("error") {
            let err: OpenRouterError = from_value(error.clone())
                .map_err(|e| ClientError::from(e))?;

            return Err(ClientError::Api(err));
        }

        let chat_res: ChatResponse = from_value(json)
            .map_err(|e| ClientError::from(e))?;

        Ok(chat_res)
    }

    /// Send a chat completion request to a selected model. For streaming
    /// responses, see [Client::stream_chat_completion].
    ///
    /// API Reference: [Chat Completion](https://openrouter.ai/docs/api-reference/chat-completion)
    pub async fn chat_completion(&self, req: ChatRequest) -> Result<ChatResponse, ClientError> {
        let req = req.with_stream(Some(false));
        let res = self.0.post(format!("{API_ENDPOINT}/chat/completions"))
            .json(&req)
            .send().await
            .map_err(|e| ClientError::from(e))?;

        let json: Value = res.json()
            .await.map_err(|e| ClientError::from(e))?;

        if let Some(error) = json.get("error") {
            let err: OpenRouterError = from_value(error.clone())
                .map_err(|e| ClientError::from(e))?;

            return Err(ClientError::Api(err));
        }

        let chat_res: ChatResponse = from_value(json)
            .map_err(|e| ClientError::from(e))?;

        Ok(chat_res)
    }

    /// Send a completion request to a selected model, returning a SSE
    /// stream. For text-only completions, see [Client::completion].
    ///
    /// API Reference: [Completion](https://openrouter.ai/docs/api-reference/completion)
    pub async fn stream_completion(&self, req: ChatRequest) -> Result<StreamSSE, ClientError> {
        let req = req.with_stream(Some(true));
        let res = self.0.post(format!("{API_ENDPOINT}/completions"))
            .json(&req)
            .send().await
            .map_err(|e| ClientError::from(e))?;

        let stream = res.bytes_stream();
        Ok(StreamSSE::new(stream))
    }

    /// Send a chat completion request to a selected model, returning a SSE
    /// stream. For text-only completions, see [Client::completion].
    ///
    /// API Reference: [Chat Completion](https://openrouter.ai/docs/api-reference/chat-completion)
    pub async fn stream_chat_completion(&self, req: ChatRequest) -> Result<StreamSSE, ClientError> {
        let req = req.with_stream(Some(true));
        let res = self.0.post(format!("{API_ENDPOINT}/chat/completions"))
            .json(&req)
            .send().await
            .map_err(|e| ClientError::from(e))?;

        let stream = res.bytes_stream();
        Ok(StreamSSE::new(stream))
    }

    /// Fetches a list of models available through the API.
    ///
    /// API Reference: [List Available Models](https://openrouter.ai/docs/api-reference/list-available-models)
    pub async fn list_models(&self) -> Result<Vec<Model>, ClientError> {
        let res = self.0.get(format!("{API_ENDPOINT}/models"))
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
    use super::super::chat::*;

    use std::env;
    use dotenv::dotenv;

    fn get_test_client() -> Client {
        dotenv().ok();
        let token = env::var("OPENROUTER_API_KEY")
            .expect("missing OPENROUTER_API_KEY env variable");

        Client::new(&token)
    }

    #[tokio::test]
    async fn test_completion() {
        let client = get_test_client();
        let req = ChatRequest::from_prompt(
            "deepseek/deepseek-chat-v3-0324:free",
            "Hello, who are you?",
        ).with_max_tokens(Some(5));

        let res = client.completion(req).await.unwrap();
        assert!(res.id.is_some());
        assert!(res.choices.is_some());

        assert!(res.usage.is_some());
        assert!(res.usage.unwrap().completion_tokens <= 5);

        let choices = res.choices.unwrap();
        assert!(choices.len() > 0);
        assert!(choices[0].text.is_some());
    }

    #[tokio::test]
    async fn test_chat_completion() {
        let client = get_test_client();
        let messages = vec![
            Message { role: Role::User, content: "hello".to_string() }
        ];

        let req = ChatRequest::from_messages(
            "deepseek/deepseek-chat-v3-0324:free",
            messages,
        ).with_max_tokens(Some(5));

        let res = client.chat_completion(req).await.unwrap();
        assert!(res.id.is_some());
        assert!(res.choices.is_some());

        assert!(res.usage.is_some());
        assert!(res.usage.unwrap().completion_tokens == 5);

        let choices = res.choices.unwrap();
        assert!(choices.len() > 0);
        assert!(choices[0].message.is_some());
    }

    #[tokio::test]
    async fn test_api_error() {
        let client = get_test_client();
        let req = ChatRequest::from_prompt("fake model", "?");
        let res = client.completion(req).await;
        assert!(res.is_err());
        if let Err(err) = res {
            if let ClientError::Api(err) = err {
                assert!(err.code == 400);
                return;
            }
        }
        panic!("expected ClientError::Api error type");
    }

    #[tokio::test]
    async fn test_streaming() {
        let client = get_test_client();
        let req = ChatRequest::from_prompt(
            "deepseek/deepseek-chat-v3-0324:free",
            "Hello, who are you?",
        ).with_max_tokens(Some(5));

        let mut stream = client.stream_completion(req).await.unwrap();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    if let Some(choices) = response.choices {
                        if let Some(text) = &choices[0].text {
                            println!("Streamed text: {}", text);
                        }
                    }
                }
                Err(e) => println!("Stream error: {}", e),
            }
        }
    }

    #[tokio::test]
    async fn test_stream_completion() {
        let client = get_test_client();
        let req = ChatRequest::from_prompt(
            "deepseek/deepseek-chat-v3-0324:free",
            "Hello, who are you?",
        ).with_max_tokens(Some(10));

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
                }
                Err(e) => {
                    println!("Stream error: {e}");
                    break;
                }
            }
        }

        assert!(response_count > 0, "Should receive at least one response");
    }

    #[tokio::test]
    async fn test_stream_chat_completion() {
        let client = get_test_client();
        let messages = vec![
            Message { role: Role::User, content: "hello".to_string() }
        ];

        let req = ChatRequest::from_messages(
            "deepseek/deepseek-chat-v3-0324:free",
            messages,
        ).with_max_tokens(Some(5));

        let mut stream = client.stream_chat_completion(req).await.unwrap();
        let mut response_count = 0;

        while let Some(result) = stream.next().await {
            match result {
                Ok(_response) => {
                    response_count += 1;
                    if response_count >= 3 {
                        break; // Limit test to avoid long execution
                    }
                }
                Err(e) => {
                    println!("Stream error: {}", e);
                    break;
                }
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
