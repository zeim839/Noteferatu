use super::errors::{ClientError, GoogleError};
use super::models::Model;
use super::StreamSSE;
use super::chat::*;

#[allow(unused)]
use tokio_stream::StreamExt;

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

    /// Generates a model response given an input
    /// [ChatRequest](super::chat::ChatRequest).
    ///
    /// API Reference: [models.generateContent](https://ai.google.dev/api/generate-content#method:-models.generatecontent)
    pub async fn completions(&self, model: &str, req: ChatRequest) -> Result<ChatResponse, ClientError> {
        let res = self.0.post(format!("{API_ENDPOINT}/models/{model}:generateContent"))
            .json(&req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: GoogleError = from_value(error.clone())?;
            return Err(ClientError::Api(err));
        }

        Ok(from_value(json)?)
    }

    /// Generates a streamed response from the model given an input
    /// [ChatRequest](super::chat::ChatRequest).
    ///
    /// API Reference: [models.streamGenerateContent](https://ai.google.dev/api/generate-content#method:-models.streamgeneratecontent)
    pub async fn stream_completions(&self, model: &str, req: ChatRequest) -> Result<StreamSSE, ClientError> {
        let res = self.0.post(format!("{API_ENDPOINT}/models/{model}:streamGenerateContent"))
            .json(&req).send().await?;

        Ok(StreamSSE::new(res.bytes_stream()))
    }

    /// Lists the Models available through the Gemini API.
    ///
    /// API Reference: [List Models](https://ai.google.dev/api/models#method:-models.list)
    pub async fn list_models(&self) -> Result<Vec<Model>, ClientError> {
        let res = self.0.get(format!("{API_ENDPOINT}/models?pageSize=1000"))
            .send().await?;

        let json: Value = res.json().await?;
        Ok(from_value(json["models"].clone())?)
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

        let res = client.completions("gemini-2.5-pro", req).await.unwrap();
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

        let mut stream = client.stream_completions("gemini-2.5-pro", req).await.unwrap();

        let mut all_text = String::new();
        while let Some(res) = stream.next().await {
            let res = res.unwrap();
            for candidate in res.candidates {
                for part in candidate.content.parts {
                    if let Some(text) = part.data.text {
                        all_text.push_str(&text);
                    }
                }
            }
        }
        assert!(!all_text.is_empty());
    }

    #[tokio::test]
    async fn test_list_models() {
        let client = get_test_client();
        let models = client.list_models().await.unwrap();
        assert!(models.len() > 0);
    }
}
