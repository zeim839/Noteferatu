use std::time::Duration;
use serde_json::{Value, from_value};
use tokio::sync::mpsc::Sender;
use tokio_stream::StreamExt;
use reqwest::header;

use super::request::Request;
use super::response::Response;
use super::model::Model;
use super::error::Error as GeminiError;
use crate::Result;

/// Google Gemini API endpoint.
const API_ENDPOINT: &str = "https://generativelanguage.googleapis.com/v1beta";

/// A Google Gemini API client.
pub struct Gemini(reqwest::Client);

impl Gemini {

    /// Create a new [Gemini] client from an API token.
    pub fn from_token(api_token: &str) -> Self {

        // Construct default request headers.
        let mut headers = header::HeaderMap::new();
        let mut auth = header::HeaderValue::from_str(api_token).unwrap();
        auth.set_sensitive(true);
        headers.insert("x-goog-api-key", auth);

        // Build HTTP client.
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(20))
            .default_headers(headers)
            .build()
            .unwrap();

        Self(client)
    }

    /// Generates a model response given an input
    /// [Request](super::request::Request).
    ///
    /// See: [API Reference](https://ai.google.dev/api/generate-content#method:-models.generatecontent)
    pub async fn generate(&self, req: &Request) -> Result<Response> {
        let model = &req.model;
        let res = self.0.post(format!("{API_ENDPOINT}/models/{model}:generateContent"))
            .json(&req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: GeminiError = from_value(error.clone())?;
            return Err(err.into());
        }

        Ok(from_value(json)?)
    }

    /// Stream a model response given an input
    /// [Request](super::request::Request).
    ///
    /// See: [API Reference](https://ai.google.dev/api/generate-content#method:-models.generatecontent)
    pub async fn stream(&self, req: &Request, pipe: Sender<Response>) -> Result<()> {
        let model = &req.model;
        let res = self.0.post(format!("{API_ENDPOINT}/models/{model}:streamGenerateContent"))
            .json(&req).send().await?;

        if res.status().is_success() {
            let mut buffer = String::new();
            let mut stream = res.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                buffer.push_str(&String::from_utf8_lossy(&chunk));
                while let Some(event) = Self::parse_event(&mut buffer) {
                    // Pipe was most likely intentionally closed;
                    if pipe.send(event).await.is_err() {
                        return Ok(());
                    }
                }
            }
            return Ok(());
        }

        let json: Value = res.json().await?;
        let err = json.get("error")
            .map(|v| from_value::<GeminiError>(v.clone()).unwrap_or_default())
            .unwrap_or_default();

        Err(err.into())
    }

    /// Lists the models available through the Gemini API.
    ///
    /// See: [API Reference](https://ai.google.dev/api/models#method:-models.list)
    pub async fn list_models(&self) -> Result<Vec<Model>> {
        let res = self.0.get(format!("{API_ENDPOINT}/models?pageSize=1000"))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: GeminiError = from_value(error.clone())?;
            return Err(err.into());
        }

        Ok(from_value(json["models"].clone())?)
    }

    /// Gets information about a specific model.
    ///
    /// See: [API Reference](https://ai.google.dev/api/models#method:-models.get)
    pub async fn get_model(&self, name: &str) -> Result<Model> {
        let res = self.0.get(format!("{API_ENDPOINT}/{name}"))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: GeminiError = from_value(error.clone())?;
            return Err(err.into());
        }

        Ok(from_value(json)?)
    }

    /// Parses a Gemini SSE event.
    fn parse_event(buffer: &mut String) -> Option<Response> {
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
                        match serde_json::from_str::<Response>(json_str_to_parse) {
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
                                // The object is malformed. We'll drain up to what we
                                // thought was the end of the object and try again.
                                buffer.drain(..=i);
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
    use crate as renfield;
    use crate::client::gemini::*;
    use std::env;
    use dotenv::dotenv;
    use once_cell::sync::Lazy;

    static CLIENT: Lazy<Gemini> = Lazy::new(|| {
        dotenv().ok();
        let token = env::var("GEMINI_API_KEY")
            .expect("missing GEMINI_API_KEY");

        Gemini::from_token(&token)
    });

    #[tokio_shared_rt::test(shared)]
    async fn test_api_error() {
        let err = Gemini::from_token("bad-fake-token")
            .list_models().await.unwrap_err();

        match err {
            crate::Error::Client(err) => match err {
                crate::client::Error::Gemini(err) => {
                    assert_eq!(err.code, 400);
                    assert_eq!(err.status, "INVALID_ARGUMENT");
                },
                _ => panic!("returned error from wrong client"),
            },
            _ => panic!("returned wrong error variant"),
        }
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate() {
        let req = Request::from_model("gemini-2.5-flash")
            .system_instruction(msg!("user", "you are a helpful assistant"))
            .push_content(msg!("user", "hello, who are you?"))
            .thinking_config(ThinkingConfig::new(false, Some(0)))
            .max_output_tokens(20);

        let mut res = CLIENT.generate(&req).await.unwrap();
        assert!(res.candidates.len() > 0);
        assert!(res.usage_metadata.candidates_token_count.unwrap() > 0);

        let content = res.candidates.remove(0).content;
        assert_eq!(content.role, Some(Role::Model));
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_tool_calling() {

        // Ignore "unused fields" warnings.
        #![allow(dead_code)]

        use crate::tools::{Tool as _, tool, schema};

        #[derive(tool, schema)]
        #[tool(rename = "get_weather")]
        #[tool(desc = "get the weather at a specific location")]
        struct GetWeather {

            /// The city name followed by country, e.g. Paris, France.
            pub location: String,
        }

        let req = Request::from_model("gemini-2.5-flash")
            .push_content(msg!("user", "what is the current weather in Paris, France"))
            .push_tool(Tool::from(GetWeather::as_gemini_tool()))
            .thinking_config(ThinkingConfig::new(true, Some(50)))
            .max_output_tokens(200);

        let res = CLIENT.generate(&req).await.unwrap();
        assert!(res.candidates.len() > 0);

        let mut has_tool_call = false;
        'outer: for candidate in res.candidates {
            for part in candidate.content.parts {
                match part.data {
                    Data::FunctionCall(call) => {
                        if call.name == "get_weather" {
                            has_tool_call = true;
                            break 'outer;
                        }
                    },
                    _ => {},
                }
            }
        }

        assert!(has_tool_call);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_generation_config() {
        let req = Request::from_model("gemini-2.5-flash")
            .push_content(msg!("user", "hello"))
            .thinking_config(ThinkingConfig::new(false, Some(0)))
            .push_stop_sequence("[STOP]")
            .push_response_modality(Modality::Text)
            .candidate_count(1)
            .top_p(0.1)
            .temperature(1.0)
            .seed(99)
            .enhanced_civic_answers(true)
            .max_output_tokens(20);

        let res = CLIENT.generate(&req).await.unwrap();
        assert!(res.usage_metadata.candidates_token_count.unwrap() > 0);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_reasoning() {
        let req = Request::from_model("gemini-2.5-flash")
            .push_content(msg!("user", "what is 5+7?"))
            .thinking_config(ThinkingConfig::new(true, Some(20)))
            .max_output_tokens(50);

        let mut res = CLIENT.generate(&req).await.unwrap();
        let mut has_reasoning = false;
        let candidate = res.candidates.remove(0);
        for part in candidate.content.parts {
            if part.thought.is_some_and(|t| t) {
                has_reasoning = true;
                break;
            }
        }

        assert!(has_reasoning);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_web_search() {
        let req = Request::from_model("gemini-2.5-flash")
            .push_content(msg!("user", "who won the euro 2024?"))
            .thinking_config(ThinkingConfig::new(false, Some(0)))
            .push_tool(Tool::google_search(None))
            .max_output_tokens(100);

        let res = CLIENT.generate(&req).await.unwrap();

        // Web searches are not returned as a FunctionResponse, they
        // just augment the generated content.
        if let Some(tool_tokens) = res.usage_metadata.tool_use_prompt_token_count {
            assert_ne!(tool_tokens, 0);
        }
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_code_execution() {
        let req = Request::from_model("gemini-2.5-flash")
            .push_content(msg!("user", "What is the sum of the first 50 prime numbers?"))
            .push_content(msg!("user", "Generate and run code for the calculation, and make sure you get all 50."))
            .max_output_tokens(250)
            .push_tool(Tool::code_execution());

        let mut res = CLIENT.generate(&req).await.unwrap();

        if let Some(tool_tokens) = res.usage_metadata.tool_use_prompt_token_count {
            assert_ne!(tool_tokens, 0);
        }

        let mut has_executable_code = false;
        let candidate = res.candidates.remove(0);
        for _part in candidate.content.parts {
            if matches!(Data::ExecutableCode, _part) {
                has_executable_code = true;
                break;
            }
        }

        assert!(has_executable_code);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_google_maps() {
        let req = Request::from_model("gemini-2.5-flash")
            .push_content(msg!("user", "What are the best Italian restaurants within a 15-minute walk from here?"))
            // Location context (coordinates are a place in Los Angeles).
            .retrieval_config(RetrievalConfig::new(34.050481, -118.248526))
            .push_tool(Tool::google_maps(None));

        let res = CLIENT.generate(&req).await.unwrap();

        // Does not return FunctionCall/FunctionResponse, only
        // augments model context.
        if let Some(tool_tokens) = res.usage_metadata.tool_use_prompt_token_count {
            assert_ne!(tool_tokens, 0);
        }
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_url_context_retrieval() {
        let url = "https://www.foodnetwork.com/recipes/ina-garten/perfect-roast-chicken-recipe-1940592";
        let msg = format!("What is the cooking time for {url}?");
        let req = Request::from_model("gemini-2.5-flash")
            .push_content(msg!("user", msg))
            .push_tool(Tool::url_context())
            .max_output_tokens(250);

        let res = CLIENT.generate(&req).await.unwrap();
        if let Some(tool_tokens) = res.usage_metadata.tool_use_prompt_token_count {
            assert_ne!(tool_tokens, 0);
        }
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_input_file_data() {
        use base64::Engine;
        use base64::prelude::BASE64_STANDARD;

        // Downloads a PDF and feeds it to the model as base64
        // inline media bytes.
        let url = "https://discovery.ucl.ac.uk/id/eprint/10089234/1/343019_3_art_0_py4t4l_convrt.pdf";
        let bytes = reqwest::get(url).await.unwrap()
            .bytes().await.unwrap();

        let base64 = BASE64_STANDARD.encode(bytes);
        let req = Request::from_model("gemini-2.5-flash")
            .push_content(msg!("user", Blob::new("application/pdf", &base64)))
            .thinking_config(ThinkingConfig::new(false, Some(0)))
            .max_output_tokens(50);

        let mut res = CLIENT.generate(&req).await.unwrap();
        let candidate = res.candidates.remove(0);

        let mut has_response = false;
        for part in candidate.content.parts {
            if matches!(part.data, Data::Text(_)) {
                has_response = true;
                break;
            }
        }

        assert!(has_response);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_tts() {
        let req = Request::from_model("gemini-2.5-flash-preview-tts")
            .push_content(msg!("user", "Say cheerfully: Have a wonderful day!"))
            .speech_config(VoiceConfig::from("Zephyr").into())
            .push_response_modality(Modality::Audio);

        let res = CLIENT.generate(&req).await.unwrap();
        assert!(!res.candidates.is_empty());

        let mut contains_audio = false;
        'outer: for candidate in res.candidates {
            for part in candidate.content.parts {
                match part.data {
                    Data::InlineData(blob) => {
                        if blob.mime_type.contains("audio") {
                            contains_audio = true;
                            break 'outer;
                        }
                    },
                    _ => continue,
                }
            }
        }

        assert!(contains_audio);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_tts_multi_speaker() {
        let joe = SpeakerVoiceConfig{
            speaker: "Joe".to_string(),
            voice_config: VoiceConfig::from("Kore"),
        };

        let jane = SpeakerVoiceConfig{
            speaker: "Jane".to_string(),
            voice_config: VoiceConfig::from("Puck"),
        };

        // gemini-2.5-flash-preview-tts does not support multi-turn
        // conversations. i.e. `push_content` can only be called once.
        let req = Request::from_model("gemini-2.5-flash-preview-tts")
            .push_content(msg!(
                "user",
                "TTS the following conversation:",
                "Joe: How's it going today Jane?",
                "Jane: Not too bad, how about you?"
            ))
            .push_response_modality(Modality::Audio)
            .speech_config(MultiSpeakerVoiceConfig::from(vec![joe, jane]).into());

        let res = CLIENT.generate(&req).await.unwrap();
        assert!(!res.candidates.is_empty());

        let mut contains_audio = false;
        'outer: for candidate in res.candidates {
            for part in candidate.content.parts {
                match part.data {
                    Data::InlineData(blob) => {
                        if blob.mime_type.contains("audio") {
                            contains_audio = true;
                            break 'outer;
                        }
                    },
                    _ => continue,
                }
            }
        }

        assert!(contains_audio);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_audio_understanding() {
        use base64::Engine;
        use base64::prelude::BASE64_STANDARD;

        // Fetch an audio file and convert it to a base64 string
        // See: https://platform.openai.com/docs/guides/audio?example=audio-in
        let url = "https://cdn.openai.com/API/docs/audio/alloy.wav";
        let bytes = reqwest::get(url).await.unwrap()
            .bytes().await.unwrap();

        let base64 = BASE64_STANDARD.encode(bytes);
        let req = Request::from_model("gemini-2.5-flash")
            .push_content(msg!("user", "Transcribe this audio message", Blob::new("audio/wav", &base64)))
            .max_output_tokens(100)
            .thinking_config(ThinkingConfig::new(false, Some(0)));

        let mut res = CLIENT.generate(&req).await.unwrap();
        let candidate = res.candidates.remove(0);

        let mut has_response = false;
        for part in candidate.content.parts {
            if matches!(part.data, Data::Text(_)) {
                has_response = true;
                break;
            }
        }

        assert!(has_response);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_stream() {
        let req = Request::from_model("gemini-2.5-flash")
            .push_content(msg!("user", "hello, who are you?"))
            .thinking_config(ThinkingConfig::new(false, Some(0)))
            .max_output_tokens(20);

        let (tx, mut rx) = tokio::sync::mpsc::channel::<Response>(4);
        let handle = tokio::spawn(async move {
            CLIENT.stream(&req, tx).await.unwrap();
        });

        let mut completion_tokens = 0;
        while let Some(res) = rx.recv().await {
            if let Some(tokens) = res.usage_metadata.candidates_token_count {
                completion_tokens += tokens;
            }
        }

        assert!(handle.await.is_ok());
        assert_ne!(completion_tokens, 0);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_list_models() {
        let models = CLIENT.list_models().await.unwrap();
        assert!(models.len() > 0);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_get_model() {
        let models = CLIENT.list_models().await.unwrap();
        assert!(models.len() > 0);

        let model = CLIENT.get_model(&models[0].name).await.unwrap();
        assert!(model.name == models[0].name);
    }
}
