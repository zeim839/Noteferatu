use std::time::Duration;
use serde_json::{Value, from_value};
use tokio::sync::mpsc::Sender;
use tokio_stream::StreamExt;
use reqwest::header;

use super::request::Request;
use super::response::Response;
use super::model::Model;
use super::error::Error as OpenAIError;
use crate::Result;

/// OpenAI API endpoint.
const API_ENDPOINT: &str = "https://api.openai.com/v1";

/// OpenAI chat completions client.
pub struct OpenAI(reqwest::Client);

impl OpenAI {

    /// Create a new [OpenAI] client from an API token.
    pub fn from_token(api_token: &str) -> Self {

        // Construct default request headers.
        let mut headers = header::HeaderMap::new();
        let mut auth = header::HeaderValue::from_str(&format!("Bearer {api_token}")).unwrap();
        auth.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth);

        // Build HTTP client.
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(20))
            .default_headers(headers)
            .build()
            .unwrap();

        Self(client)
    }

    /// Create a new chat completion (non-streaming).
    ///
    /// Creates a model response for the given chat conversation.
    ///
    /// See: [API Reference](https://platform.openai.com/docs/api-reference/chat/create)
    pub async fn chat_completion(&self, req: &Request) -> Result<Response> {
        let res = self.0.post(format!("{API_ENDPOINT}/chat/completions"))
            .json(req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenAIError = from_value(error.clone())?;
            return Err(err.into());
        }

        Ok(from_value(json)?)
    }

    /// Stream a chat completion.
    ///
    /// See: [API Reference](https://platform.openai.com/docs/api-reference/chat/create)
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
            .map(|value| from_value::<OpenAIError>(value.clone()).unwrap_or_default())
            .unwrap_or_default();

        return Err(err.into());
    }

    /// List the currently available models.
    ///
    /// Lists the currently available models, and provides basic
    /// information about each one such as the owner and availability.
    ///
    /// See: [API Reference](https://platform.openai.com/docs/api-reference/models/list)
    pub async fn list_models(&self) -> Result<Vec<Model>> {
        let res = self.0.get(format!("{API_ENDPOINT}/models"))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenAIError = from_value(error.clone())?;
            return Err(err.into());
        }

        let models: Vec<Model> = from_value(json["data"].clone())?;
        Ok(models)
    }

    /// Retrieve an OpenAI model.
    ///
    /// Retrieves a model instance, providing basic information about
    /// the model such as the owner and permissioning.
    ///
    /// See: [API Reference](https://platform.openai.com/docs/api-reference/models/retrieve)
    pub async fn get_model(&self, model_id: &str) -> Result<Model> {
        let res = self.0.get(format!("{API_ENDPOINT}/models/{model_id}"))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: OpenAIError = from_value(error.clone())?;
            return Err(err.into());
        }

        Ok(from_value(json)?)
    }

    /// Parses an OpenAI SSE event.
    pub(crate) fn parse_event(buffer: &mut String) -> Option<Response> {
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
    use crate::client::openai::*;
    use std::env;
    use dotenv::dotenv;

    fn get_test_client() -> OpenAI {
        dotenv().ok();
        let token = env::var("OPENAI_API_KEY")
            .expect("missing OPENAI_API_KEY");

        OpenAI::from_token(&token)
    }

    #[tokio::test]
    async fn test_api_error() {
        let res = OpenAI::from_token("bad-token").list_models().await;
        assert!(res.is_err());
        match res.unwrap_err() {
            crate::Error::Client(err) => match err {
                crate::client::Error::OpenAI(err) => {
                    assert!(err.kind == "invalid_request_error");
                    assert!(err.code == "invalid_api_key");
                },
                _ => panic!("received a different client's API response object"),
            },
            _ => panic!("unexpected error type"),
        }
    }

    #[tokio::test]
    async fn test_chat_completion() {
        let req = Request::from_model("gpt-4.1-mini")
            .push_message(msg!("user", "hello"))
            .max_completion_tokens(10);

        let res = get_test_client().chat_completion(&req)
            .await.unwrap();

        assert!(res.choices.len() > 0);
        assert!(res.usage.unwrap().completion_tokens > 0);
        assert!(matches!(res.choices[0].clone().message.unwrap(), Message::Assistant{ .. }));
    }

    #[tokio::test]
    async fn test_audio_output_completion() {
        let req = Request::from_model("gpt-4o-audio-preview")
            .append_modalities(&mut vec![Modality::Text, Modality::Audio])
            .audio(AudioConfig::default())
            .push_message(msg!("user", "say hello!"))
            .max_completion_tokens(50);

        let res = get_test_client().chat_completion(&req)
            .await.unwrap();

        assert!(res.choices.len() > 0);
        assert!(res.usage.unwrap().completion_tokens > 0);
        if let Message::Assistant{ audio, .. } = res.choices[0].clone().message.unwrap() {
            assert!(audio.is_some());
            return;
        }

        panic!("no audio response returned");
    }

    #[tokio::test]
    async fn test_audio_input_completion() {

        use base64::Engine;
        use base64::prelude::BASE64_STANDARD;

        // Fetch an audio file and convert it to a base64 string
        // See: https://platform.openai.com/docs/guides/audio?example=audio-in
        let url = "https://cdn.openai.com/API/docs/audio/alloy.wav";
        let bytes = reqwest::get(url).await.unwrap()
            .bytes().await.unwrap();

        let base64 = BASE64_STANDARD.encode(bytes);
        let req = Request::from_model("gpt-4o-audio-preview")
            .push_message(msg!(
                "user",
                "summarize this audio",
                AudioInput::new(&base64, AudioFormat::Wav)
            ))
            .max_completion_tokens(10);

        let res = get_test_client().chat_completion(&req)
            .await.unwrap();

        assert!(res.choices.len() > 0);
        assert!(res.usage.unwrap().completion_tokens > 0);
        if let Message::Assistant { content, .. } = res.choices[0].clone().message.unwrap() {
            assert!(content.is_some());
            return;
        }

        panic!("unexpected response");
    }

    #[tokio::test]
    async fn test_image_input_completion() {
        let url = "https://upload.wikimedia.org/wikipedia/commons/5/57/Pelium_Man%C5%93uvre.jpg";
        let req = Request::from_model("gpt-4o")
            .push_message(msg!("user", "what's in this image?", ImageInput::from_url(url)))
            .max_completion_tokens(10);

        let res = get_test_client().chat_completion(&req)
            .await.unwrap();

        assert!(res.choices.len() > 0);
        assert!(res.usage.unwrap().completion_tokens > 0);
        if let Message::Assistant { content, .. } = res.choices[0].clone().message.unwrap() {
            assert!(content.is_some());
            return;
        }

        panic!("unexpected response");
    }

    #[tokio::test]
    async fn test_file_input_completion() {
        let base64 = "data:application/pdf;base64,SGVsbG8sIFdvcmxkIQ==";
        let req = Request::from_model("gpt-4o-mini")
            .push_message(msg!("user", "what's in this file?", FileInput::from_file_data("my-file.pdf", &base64)))
            .max_completion_tokens(10);

        let res = get_test_client().chat_completion(&req)
            .await.unwrap();

        assert!(res.choices.len() > 0);
        assert!(res.usage.unwrap().completion_tokens > 0);
        if let Message::Assistant { content, .. } = res.choices[0].clone().message.unwrap() {
            assert!(content.is_some());
            return;
        }

        panic!("unexpected response");
    }

    #[tokio::test]
    async fn test_function_tool_call_completion() {

        use crate::tools::{Tool, tool, schema};

        #[derive(tool, schema)]
        #[tool(rename = "get_weather")]
        #[tool(desc = "get the weather at a specific location")]
        struct GetWeather {

            /// The city name followed by country, e.g. Bogota,
            /// Columbia.
            pub location: String,
        }

        let req = Request::from_model("gpt-4o-mini")
            .push_message(msg!("user", "what's the weather in Bogota, Columbia?"))
            .push_tool(GetWeather::as_openai_tool())
            .max_completion_tokens(50);

        let res = get_test_client().chat_completion(&req)
            .await.unwrap();

        assert!(res.choices.len() > 0);
        assert!(res.usage.unwrap().completion_tokens > 0);
        if let Message::Assistant { tool_calls, .. } = res.choices[0].clone().message.unwrap() {
            if let Some(tool_calls) = tool_calls {
                if let ToolCall::Function { function, .. } = &tool_calls[0] {
                    assert_eq!(function.name, "get_weather");
                    return;
                }
            }
        }

        panic!("unexpected response");
    }

    #[tokio::test]
    async fn test_stream_completion() {
        let req = Request::from_model("gpt-4.1-mini")
            .push_message(msg!("user", "hello"))
            .stream_options(StreamOptions::new(true))
            .max_completion_tokens(10)
            .stream(true);

        let (tx, mut rx) = tokio::sync::mpsc::channel::<Response>(4);
        let handle = tokio::spawn(async move {
            get_test_client().stream_chat_completion(&req, tx).await.unwrap();
        });

        let mut completion_tokens = 0;
        while let Some(res) = rx.recv().await {
            if let Some(usage) = res.usage {
                completion_tokens += usage.completion_tokens;
            }
        }

        assert!(handle.await.is_ok());
        assert!(completion_tokens > 0);
    }

    #[tokio::test]
    async fn test_list_models() {
        let models = get_test_client().list_models().await.unwrap();
        assert!(models.len() > 0);
    }

    #[tokio::test]
    async fn test_get_model() {
        let client = get_test_client();
        let models = client.list_models().await.unwrap();
        assert!(models.len() > 0);

        let model = client.get_model(&models[0].id).await.unwrap();
        assert!(model.id == models[0].id);
    }
}
