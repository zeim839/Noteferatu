use std::time::Duration;
use serde_json::{Value, from_value};
use tokio::sync::mpsc::Sender;
use tokio_stream::StreamExt;
use reqwest::header;

use super::model::Model;
use super::request::Request;
use super::response::Response;
use super::error::Error as OpenRouterError;
use crate::Result;

/// OpenRouter API endpoint.
const API_ENDPOINT: &str = "https://openrouter.ai/api/v1";

/// OpenRouter chat completions client.
pub struct OpenRouter(reqwest::Client);

impl OpenRouter {

    /// Create a new [OpenRouter] client from an API token.
    pub fn from_token(api_token: &str) -> Self {

        // Construct default request headers.
        let mut headers = header::HeaderMap::new();
        let mut auth = header::HeaderValue::from_str(&format!("Bearer {api_token}")).unwrap();
        auth.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth);

        // Build HTTP client.
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(30))
            .default_headers(headers)
            .build()
            .unwrap();

        Self(client)
    }

    /// Send a chat completion request to a selected model.
    ///
    /// API Reference: [Chat Completion](https://openrouter.ai/docs/api-reference/chat-completion)
    pub async fn chat_completion(&self, req: &Request) -> Result<Response> {
        let res = self.0.post(format!("{API_ENDPOINT}/chat/completions"))
            .json(req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenRouterError = from_value(error.clone())?;
            return Err(err.into());
        }

        Ok(from_value(json)?)
    }

    /// Send a streaming chat completion request to a selected model.
    ///
    /// API Reference: [Chat Completion](https://openrouter.ai/docs/api-reference/chat-completion)
    pub async fn stream_chat_completion(&self, req: &Request, pipe: Sender<Response>) -> Result<()> {
        let res = self.0.post(format!("{API_ENDPOINT}/chat/completions"))
            .json(req).send().await?;

        if res.status().is_success() {
            let mut buffer = String::new();
            let mut stream = res.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                buffer.push_str(&String::from_utf8_lossy(&chunk));
                while let Some(event) = Self::parse_event(&mut buffer) {
                    // Pipe was most likely intentionally closed.
                    if let Err(_) = pipe.send(event).await {
                        return Ok(());
                    }
                }
            }
            return Ok(());
        }

        let json: Value = res.json().await?;
        let err = json.get("error")
            .map(|value| from_value::<OpenRouterError>(value.clone()).unwrap_or_default())
            .unwrap_or_default();

        return Err(err.into());
    }

    /// Fetches a list of models available via the API.
    ///
    /// API Reference: [List Available Models](https://openrouter.ai/docs/api-reference/list-available-models)
    pub async fn list_models(&self) -> Result<Vec<Model>> {
        let res = self.0.get(format!("{API_ENDPOINT}/models"))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenRouterError = from_value(error.clone())?;
            return Err(err.into());
        }

        let models: Vec<Model> = from_value(json["data"].clone())?;
        Ok(models)
    }

    /// Parses an OpenRouter SSE event.
    #[inline]
    fn parse_event(buffer: &mut String) -> Option<Response> {
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

#[cfg(test)]
mod tests {
    use crate as renfield;
    use crate::client::openrouter::*;
    use std::env;
    use dotenv::dotenv;
    use once_cell::sync::Lazy;
    use crate::tools::{Tool, tool, schema};

    static CLIENT: Lazy<OpenRouter> = Lazy::new(|| {
        dotenv().ok();
        let token = env::var("OPENROUTER_API_KEY")
            .expect("missing OPENROUTER_API_KEY");

        OpenRouter::from_token(&token)
    });

    #[tokio_shared_rt::test(shared)]
    async fn test_api_error() {
        let req = Request::from_model("nonexistent-model")
            .push_message(msg!("user", "hello"))
            .max_completion_tokens(10);

        let err = CLIENT.chat_completion(&req).await.unwrap_err();
        match err {
            crate::Error::Client(err) => match err {
                crate::client::Error::OpenRouter(err) => {
                    assert!(err.message == "nonexistent-model is not a valid model ID");
                    assert!(err.code == 400);
                },
                _ => panic!("received a different client's API response"),
            },
            _ => panic!("unexpected error type"),
        }
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_chat_completion() {
        let req = Request::from_model("qwen/qwen3-coder:free")
            .push_message(msg!("user", "hello"))
            .max_completion_tokens(10);

        let mut res = CLIENT.chat_completion(&req)
            .await.unwrap();

        assert!(res.choices.len() > 0);
        assert!(res.usage.unwrap().completion_tokens > 0);
        let choice = res.choices.remove(0);
        assert!(matches!(choice.message.unwrap(), Message::Assistant{ .. }));
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_completion_image_input() {
        let url = "https://upload.wikimedia.org/wikipedia/commons/5/57/Pelium_Man%C5%93uvre.jpg";
        let req = Request::from_model("nvidia/nemotron-nano-12b-v2-vl:free")
            .push_message(msg!("user", "what's in this image?", ImageInput::from_url(&url)))
            .max_completion_tokens(10);

        let mut res = CLIENT.chat_completion(&req)
            .await.unwrap();

        assert!(res.choices.len() > 0);
        assert!(res.usage.unwrap().completion_tokens > 0);
        let choice = res.choices.remove(0);
        if let Message::Assistant { content, .. } = choice.message.unwrap() {
            assert!(content.is_some());
            return;
        }

        panic!("unexpected response");
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_completion_audio_input() {
        use base64::Engine;
        use base64::prelude::BASE64_STANDARD;

        // Fetch an audio file and convert it to a base64 string
        // See: https://platform.openai.com/docs/guides/audio?example=audio-in
        let url = "https://cdn.openai.com/API/docs/audio/alloy.wav";
        let bytes = reqwest::get(url).await.unwrap()
            .bytes().await.unwrap();

        let base64 = BASE64_STANDARD.encode(bytes);
        let req = Request::from_model("google/gemini-2.0-flash-001")
            .push_message(msg!("user", "summarize this audio", AudioInput::new(&base64, AudioFormat::Wav)))
            .max_completion_tokens(10);

        let mut res = CLIENT.chat_completion(&req)
            .await.unwrap();

        assert!(res.choices.len() > 0);
        assert!(res.usage.unwrap().completion_tokens > 0);
        let choice = res.choices.remove(0);
        if let Message::Assistant { content, .. } = choice.message.unwrap() {
            assert!(content.is_some());
            return;
        }

        panic!("unexpected response");
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_completion_tool_calling() {

        // Ignore "unused fields" warnings.
        #![allow(dead_code)]

        /// Get the current weather.
        #[derive(tool, schema)]
        #[tool(rename = "get_weather")]
        struct GetWeather {

            /// The city, followed by country (e.g. Paris, France).
            location: String,
        }

        let req = Request::from_model("minimax/minimax-m2:free")
            .push_message(msg!("user", "what's the weather in Paris, France?"))
            .push_tool(GetWeather::as_openrouter_tool())
            .reasoning_effort(ReasoningEffort::Minimal)
            .tool_choice_mode(ToolChoiceMode::Required);

        let mut res = CLIENT.chat_completion(&req)
            .await.unwrap();

        assert!(res.choices.len() > 0);
        let choice = res.choices.remove(0);
        if let Message::Assistant { tool_calls, .. } = choice.message.unwrap() {
            assert!(tool_calls.as_ref().is_some_and(|c| c.len() > 0));
            let tool_call = tool_calls.unwrap().remove(0);
            let FunctionCall::Function { name, arguments } = tool_call.function;
            assert_eq!(name, "get_weather");
            assert_eq!(arguments, "{\"location\": \"Paris, France\"}");
            return;
        }

        panic!("no tool call returned");
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_completion_multiple_models() {
        let req = Request::from_models(vec!["qwen/qwen3-coder:free", "deepseek/deepseek-chat-v3.1:free", "openai/gpt-oss-20b:free"])
            .push_message(msg!("user", "hello"))
            .max_completion_tokens(10);

        let mut res = CLIENT.chat_completion(&req)
            .await.unwrap();

        assert!(!res.choices.is_empty());
        assert!(res.usage.unwrap().completion_tokens > 0);
        let choice = res.choices.remove(0);
        assert!(matches!(choice.message.unwrap(), Message::Assistant{ .. }));
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_completion_reasoning() {
        let req = Request::from_model("qwen/qwen3-coder:free")
            .push_message(msg!("user", "hello"))
            .max_completion_tokens(10)
            .reasoning_effort(ReasoningEffort::Minimal);

        let mut res = CLIENT.chat_completion(&req)
            .await.unwrap();

        assert!(!res.choices.is_empty());
        assert!(res.usage.unwrap().completion_tokens > 0);
        let choice = res.choices.remove(0);
        assert!(matches!(choice.message.unwrap(), Message::Assistant{ .. }));
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_stream_chat_completion() {
        let req = Request::from_model("qwen/qwen3-coder:free")
            .push_message(msg!("user", "hello"))
            .max_completion_tokens(10)
            .stream(true);

        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        CLIENT.stream_chat_completion(&req, tx).await.unwrap();

        let mut count = 0;
        while let Some(res) = rx.recv().await {
            if !res.choices.is_empty() {
                count += 1;
            }
        }

        assert!(count > 0);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_list_models() {
        let models = CLIENT.list_models().await.unwrap();
        assert!(models.len() > 0);
    }
}
