use crate::openai::{Error, OpenAIError};
use super::models::Model;
use crate::sse::SSE;
use super::chat::*;

use serde_json::{from_value, Value};
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

    /// Generates a model response given a
    /// [ChatRequest](super::chat::ChatRequest).
    ///
    /// API Reference: [models.generateContent](https://ai.google.dev/api/generate-content#method:-models.generatecontent)
    pub async fn completion(&self, model: &str, req: ChatRequest) -> Result<ChatResponse, Error> {
        let res = self.0.post(format!("{API_ENDPOINT}/models/{model}:generateContent"))
            .json(&req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenAIError = from_value(error.clone())?;
            return Err(crate::Error::Api(err));
        }

        Ok(from_value(json)?)
    }

    /// Generates a streamed response from the model given an input
    /// [ChatRequest](super::chat::ChatRequest).
    ///
    /// API Reference: [models.streamGenerateContent](https://ai.google.dev/api/generate-content#method:-models.streamgeneratecontent)
    pub async fn stream_completion(&self, model: &str, req: ChatRequest) -> Result<SSE<ChatResponse>, Error> {
        let res = self.0.post(format!("{API_ENDPOINT}/models/{model}:streamGenerateContent"))
            .json(&req).send().await?;

        Ok(SSE::new(Box::new(Self::parse_event), res.bytes_stream()))
    }

    /// Lists the Models available through the Gemini API.
    ///
    /// API Reference: [List Models](https://ai.google.dev/api/models#method:-models.list)
    pub async fn list_models(&self) -> Result<Vec<Model>, Error> {
        let res = self.0.get(format!("{API_ENDPOINT}/models?pageSize=1000"))
            .send().await?;

        let json: Value = res.json().await?;
        Ok(from_value(json["models"].clone())?)
    }

    /// Parses a Gemini SSE event.
    fn parse_event(buffer: &mut String) -> Option<ChatResponse> {
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
                        match serde_json::from_str::<ChatResponse>(json_str_to_parse) {
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

#[cfg(test)]
mod tests {
    use super::*;
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
            .with_max_output_tokens(Some(5))
            .with_thinking_config(Some(ThinkingConfig{
                include_thoughts: true,
                thinking_budget: 128,
            }));

        let req = ChatRequest::from_prompt("hi")
            .with_generation_config(Some(config));

        let res = client.completion("gemini-2.5-pro", req).await.unwrap();
        assert!(res.candidates.len() > 0);

        let candidate = &res.candidates[0];
        if let Some(txt) = &candidate.content.parts[0].data.text {
            assert!(txt.len() > 0);
        } else {
            panic!("response did not include text");
        }

        let usage_metadata = res.usage_metadata;
        assert!(usage_metadata.total_token_count > 0);
    }

    #[tokio::test]
    async fn test_stream_completion() {
        let client = get_test_client();
        let config = GenerationConfig::new()
            .with_max_output_tokens(Some(5))
            .with_thinking_config(Some(ThinkingConfig{
                include_thoughts: true,
                thinking_budget: 128,
            }));

        let req = ChatRequest::from_prompt("hi")
            .with_generation_config(Some(config));

        let mut sse = client.stream_completion("gemini-2.5-pro", req).await.unwrap();
        while let Some(event) = sse.next::<OpenAIError>().await {
            match event {
                Ok(_) => return,
                Err(e) => panic!("stream error: {e}"),
            }
        }
        panic!("no events returned from stream");
    }

    #[tokio::test]
    async fn test_list_models() {
        let client = get_test_client();
        let models = client.list_models().await.unwrap();
        assert!(models.len() > 0);
    }

    #[tokio::test]
    async fn test_tool_call() {
        let client = get_test_client();
        let tool = Tool {
            function_declarations: Some(vec![FunctionDeclaration {
                name: "get_current_weather".to_string(),
                description: "retrieves the current weather".to_string(),
                parameters: None,
            }]),
            google_search_retrieval: None,
            google_search: None,
            url_context: None,
        };

        let req = ChatRequest::from_prompt("what's the current weather like?")
            .with_tools(Some(vec![tool]));

        let res = client.completion("gemini-2.5-pro", req).await.unwrap();
        assert!(res.candidates.len() > 0);

        let candidate = &res.candidates[0];
        let part = &candidate.content.parts[0];
        assert!(part.data.function_call.is_some());

        let function_call = part.data.function_call.as_ref().unwrap();
        assert_eq!(function_call.name, "get_current_weather");
    }
}
