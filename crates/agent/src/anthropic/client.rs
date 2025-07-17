use crate::openai::{Error, OpenAIError};
use super::models::Model;
use super::messages::*;
use crate::sse::SSE;

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
    pub async fn completion(&self, req: MessageRequest) -> Result<MessageResponse, Error> {
        let req = req.with_stream(false);
        let res = self.0.post(format!("{API_ENDPOINT}/messages"))
            .json(&req).send().await?;

        let json: Value =  res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenAIError = from_value(error.clone())?;
            return Err(crate::error::Error::Api(err));
        }

        Ok(from_value(json)?)
    }

    /// Send a message request and stream the response.
    ///
    /// API Reference: [Messages](https://docs.anthropic.com/en/api/messages)
    pub async fn stream_completion(&self, req: MessageRequest) -> Result<SSE<StreamResponse>, Error> {
        let req = req.with_stream(true);
        let res = self.0.post(format!("{API_ENDPOINT}/messages"))
            .json(&req).send().await?;

        Ok(SSE::new(Box::new(Self::parse_event), res.bytes_stream()))
    }

    /// Fetches a list of models available through the API.
    ///
    /// API Reference: [List Models](https://docs.anthropic.com/en/api/models-list)
    pub async fn list_models(&self) -> Result<Vec<Model>, Error> {
        let res = self.0.get(format!("{API_ENDPOINT}/models?limit=1000"))
            .send().await?;

        let json: Value = res.json().await?;
        Ok(from_value(json["data"].clone())?)
    }

    /// Parses an Anthropic SSE event.
    fn parse_event(buffer: &mut String) -> Option<StreamResponse> {
        // Look for complete SSE events (separated by double newlines)
        while let Some(newline_pos) = buffer.find("\n") {
            let event_block = buffer[..=newline_pos].to_string();
            buffer.drain(..=newline_pos);

            // Parse the event block line by line
            for line in event_block.lines() {
                let line = line.trim();

                // Skip empty lines and non-data lines.
                if line.is_empty() || !line.starts_with("data: ") {
                    continue;
                }

                // Remove "data: " prefix.
                let data = &line[6..];

                // Try to parse as JSON
                match serde_json::from_str::<StreamResponse>(data) {
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
        let req = MessageRequest::from_prompt(TEST_MODEL, "Hello")
            .with_max_tokens(5);

        let res = client.completion(req).await.unwrap();
        assert!(res.content.len() > 0);
        assert!(res.content[0].kind == ContentKind::Text);
        assert!(res.content[0].text.as_ref().unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_stream_completion() {
        let client = get_test_client();
        let req = MessageRequest::from_prompt(TEST_MODEL, "Hello")
            .with_max_tokens(5);

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

    const TEST_TOOL: &str = "{\"type\": \"object\", \"properties\":{}}";

    #[tokio::test]
    async fn test_tool_call() {
        let client = get_test_client();
        let req = MessageRequest::from_prompt(TEST_MODEL, "What is the
    current weather like?")
            .with_thinking(Some(Thinking{
                kind: ThinkingKind::Disabled,
                budget_tokens: None,
            }))
            .with_tools(Some(vec![ToolDefinition{
                name: "get_current_weather".to_string(),
                description: "fetches the current weather".to_string(),
                input_schema: serde_json::from_str(TEST_TOOL).unwrap(),
            }]));

        let res = client.completion(req).await.unwrap();
        assert!(res.content.len() > 0);

        let mut has_tool_call = false;
        for content in res.content.iter() {
            if let ContentKind::ToolUse = content.kind {
                assert!(content.input.is_some());
                assert!(content.name.as_ref().is_some_and(
                    |name| name == "get_current_weather"
                ));
                has_tool_call = true;
                break;
            }
        }

        assert!(has_tool_call);
    }

    #[tokio::test]
    async fn test_streaming_tool_call() {
        let client = get_test_client();
        let req = MessageRequest::from_prompt(TEST_MODEL, "What is the
    current weather like?")
            .with_thinking(Some(Thinking{
                kind: ThinkingKind::Disabled,
                budget_tokens: None,
            }))
            .with_tools(Some(vec![ToolDefinition{
                name: "get_current_weather".to_string(),
                description: "fetches the current weather".to_string(),
                input_schema: serde_json::from_str(TEST_TOOL).unwrap(),
            }]));


        let mut sse = client.stream_completion(req).await.unwrap();
        let mut has_tool_call = false;
        while let Some(event) = sse.next::<OpenAIError>().await {
            let event = event.unwrap();
            if let StreamEventType::ContentBlockStart = event.kind {
                let block = event.content_block.as_ref().unwrap();
                if block.kind == ContentKind::ToolUse {
                    assert!(block.name.as_ref().is_some_and(
                        |name| name == "get_current_weather")
                    );
                    has_tool_call = true;
                    break;
                }
            }
        }

        assert!(has_tool_call);
    }
}
