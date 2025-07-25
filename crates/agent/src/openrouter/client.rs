use crate::openai::{ChatRequest, ChatResponse};
use crate::openai::Client as OAI;
use crate::openai::ErrorAPI;
use super::models::Model;
use crate::error::Error;
use crate::sse::SSE;

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

    /// Send a chat completion request to a selected model.
    ///
    /// API Reference: [Chat Completion](https://openrouter.ai/docs/api-reference/chat-completion)
    pub async fn completion(&self, req: ChatRequest) -> Result<ChatResponse, Error> {
        let req = req.with_stream(Some(false));
        let res = self.0.post(format!("{API_ENDPOINT}/chat/completions"))
            .json(&req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: ErrorAPI = from_value(error.clone())?;
            return Err(Error {
                kind: format!("OPENROUTER_{}_ERR", err.code),
                message: err.message,
            });
        }

        let chat_res: ChatResponse = from_value(json)?;
        Ok(chat_res)
    }

    /// Send a streaming chat completion request to a selected model.
    ///
    /// API Reference: [Chat Completion](https://openrouter.ai/docs/api-reference/chat-completion)
    pub async fn stream_completion(&self, req: ChatRequest) -> Result<SSE<ChatResponse>, Error> {
        let req = req.with_stream(Some(true));
        let res = self.0.post(format!("{API_ENDPOINT}/chat/completions"))
            .json(&req).send().await?;

        Ok(SSE::new(Box::new(OAI::parse_event), res.bytes_stream()))
    }

    /// Fetches a list of models available via the API.
    ///
    /// API Reference: [List Available Models](https://openrouter.ai/docs/api-reference/list-available-models)
    pub async fn list_models(&self) -> Result<Vec<Model>, Error> {
        let res = self.0.get(format!("{API_ENDPOINT}/models"))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: ErrorAPI = from_value(error.clone())?;
            return Err(Error {
                kind: format!("OPENROUTER_{}_ERR", err.code),
                message: err.message,
            });
        }

        let models: Vec<Model> = from_value(json["data"].clone())?;
        Ok(models)
    }

    /// Check if the client is connected.
    pub async fn check(&self) -> Result<(), Error> {
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
            let err: ErrorAPI = from_value(error.clone())?;
            return Err(Error {
                kind: format!("OPENROUTER_{}_ERR", err.code),
                message: err.message,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use dotenv::dotenv;
    use crate::openai;

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
        let req = ChatRequest::from_prompt(TEST_MODEL, "hi")
            .with_max_completion_tokens(Some(5));

        let res = client.completion(req).await.unwrap();
        assert!(res.choices.len() > 0);
        assert!(res.choices[0].message.is_some());

        assert!(res.usage.is_some());
        assert!(res.usage.unwrap().completion_tokens == 5);
    }


    #[tokio::test]
    async fn test_stream_completion() {
        let client = get_test_client();
        let req = ChatRequest::from_prompt(TEST_MODEL, "hi")
            .with_max_completion_tokens(Some(10));

        let mut sse = client.stream_completion(req).await.unwrap();
        let mut response_count = 0;
        while let Some(msg) = sse.next::<ErrorAPI>().await {
            match msg {
                Ok(_) => {
                    response_count += 1;
                    if response_count >= 3 {
                        break;
                    }
                },
                Err(e) => panic!("stream error: {e}"),
            }
        }

        if response_count < 1 {
            panic!("expected at least one response");
        }
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

    #[tokio::test]
    async fn test_tool_calling() {
        let client = get_test_client();
        let req = ChatRequest::from_prompt(TEST_MODEL, "what's the current weather like?")
            .with_tools(Some(vec![openai::ToolDefinition{
                kind: "function".to_string(),
                function: openai::FunctionDefinition{
                    name: "get_current_weather".to_string(),
                    description: Some("retrieves the current weather".to_string()),
                    parameters: None,
                    strict: None,
                }
            }]));

        let res = client.completion(req).await.unwrap();
        assert!(res.choices.len() > 0);

        let choice = &res.choices[0];
        assert!(*choice.finish_reason.as_ref().unwrap() ==
                openai::FinishReason::ToolCalls
        );

        assert!(choice.message.as_ref().is_some());

        let msg = choice.message.as_ref().unwrap();
        assert!(msg.tool_calls.is_some());
        assert!(msg.tool_calls.as_ref().unwrap()[0].function.name == "get_current_weather");
    }
}
