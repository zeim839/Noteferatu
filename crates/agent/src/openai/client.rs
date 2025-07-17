use super::error::{Error, OpenAIError};
use super::models::Model;
use crate::sse::SSE;
use super::chat::*;

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
    pub async fn completion(&self, req: ChatRequest) -> Result<ChatResponse, Error> {
        let req = req.with_stream(Some(false));
        let res = self.0.post(format!("{API_ENDPOINT}/chat/completions"))
            .json(&req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenAIError = from_value(error.clone())?;
            return Err(crate::error::Error::Api(err));
        }

        Ok(from_value(json)?)
    }

    /// Send a completion request to a selected model, returning a SSE
    /// stream. For non-streaming completions, see [Client::completion].
    ///
    /// API Reference: [Stream Chat Completions](https://platform.openai.com/docs/api-reference/chat-streaming)
    pub async fn stream_completion(&self, req: ChatRequest) -> Result<SSE<ChatResponse>, Error> {
        let req = req.with_stream(Some(true));
        let res = self.0.post(format!("{API_ENDPOINT}/chat/completions"))
            .json(&req).send().await?;

        Ok(SSE::new(Box::new(Self::parse_event), res.bytes_stream()))
    }

    /// List and describe the various models available in the API.
    ///
    /// API Reference: [Models](https://platform.openai.com/docs/api-reference/models)
    pub async fn list_models(&self) -> Result<Vec<Model>, Error> {
        let res = self.0.get(format!("{API_ENDPOINT}/models"))
            .send().await?;

        let json: Value = res.json().await?;
        Ok(from_value(json["data"].clone())?)
    }

    /// Parses an OpenAI SSE event.
    pub fn parse_event(buffer: &mut String) -> Option<ChatResponse> {
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
                match serde_json::from_str::<ChatResponse>(data) {
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
        assert!(msg.content.as_ref().is_some());
    }

    #[tokio::test]
    async fn test_stream_completion() {
        let client = get_test_client();
        let req = ChatRequest::from_prompt("gpt-4.1-mini", "hi")
            .with_max_completion_tokens(Some(5));

        let mut sse = client.stream_completion(req).await.unwrap();
        let mut response_count = 0;
        while let Some(msg) = sse.next::<OpenAIError>().await {
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
    async fn test_tool_calling() {
        let client = get_test_client();
        let req = ChatRequest::from_prompt("gpt-4.1-mini", "what's the current weather like?")
            .with_tools(Some(vec![ToolDefinition{
                kind: "function".to_string(),
                function: FunctionDefinition{
                    name: "get_current_weather".to_string(),
                    description: Some("retrieves the current weather".to_string()),
                    parameters: None,
                    strict: None,
                },
            }]));

        let res = client.completion(req).await.unwrap();
        assert!(res.choices.len() > 0);

        let choice = &res.choices[0];
        assert!(*choice.finish_reason.as_ref().unwrap() ==
                FinishReason::ToolCalls
        );

        assert!(choice.message.as_ref().is_some());

        let msg = choice.message.as_ref().unwrap();
        assert!(msg.tool_calls.is_some());
        assert!(msg.tool_calls.as_ref().unwrap()[0].function.name == "get_current_weather");
    }
}
