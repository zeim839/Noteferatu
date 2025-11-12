use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio_stream::StreamExt;
use serde_json::{Value, from_value};
use reqwest::header;

use super::request::Request;
use super::response::Response;
use super::model::Model;
use super::error::Error as AnthropicError;
use crate::Result;

/// Anthropic API endpoint.
const API_ENDPOINT: &str = "https://api.anthropic.com/v1";

/// Anthropic API client.
pub struct Anthropic(reqwest::Client);

impl Anthropic {

    /// Create an [Anthropic] client using an API token.
    pub fn from_token(api_token: &str) -> Self {

        // Construct default request headers.
        let mut headers = header::HeaderMap::new();
        let version = header::HeaderValue::from_str("2023-06-01").unwrap();
        let mut key = header::HeaderValue::from_str(api_token).unwrap();
        key.set_sensitive(true);

        // Enable beta web_fetch tool.
        let web_fetch = header::HeaderValue::from_str("web-fetch-2025-09-10").unwrap();

        // Create headers.
        headers.insert("x-api-key", key);
        headers.insert("anthropic-version", version);
        headers.insert("anthropic-beta", web_fetch);

        // Build HTTP client.
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(20))
            .default_headers(headers)
            .build()
            .unwrap();

        Self(client)
    }

    /// Generate a message completion.
    pub async fn generate(&self, req: &Request) -> Result<Response> {
        let res = self.0.post(format!("{API_ENDPOINT}/messages"))
            .json(req).send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: AnthropicError = from_value(error.clone())?;
            return Err(err.into());
        }

        Ok(from_value(json)?)
    }

    /// Stream a message completion.
    pub async fn stream(&self, _req: &Request, _pipe: Sender<Response>) -> Result<()> {
        // let res = self.0.post(format!("{API_ENDPOINT}/messages"))
        //     .json(req).send().await?;

        // if res.status().is_success() {
        //     let mut buffer = String::new();
        //     let mut stream = res.bytes_stream();
        //     while let Some(chunk) = stream.next().await {
        //         let chunk = chunk?;
        //         buffer.push_str(&String::from_utf8_lossy(&chunk));
        //         while let Some(event) = Self::parse_event(&mut buffer) {
        //             // Pipe was most likely intentionally closed.
        //             if let Err(_) = pipe.send(event).await {
        //                 return Ok(());
        //             }
        //         }
        //     }
        // }

        // let json: Value = res.json().await?;
        // let err = json.get("error")
        //     .map(|v| from_value::<AnthropicError>(v.clone()).unwrap_or_default())
        //     .unwrap_or_default();

        // Err(err.into())
        todo!();
    }

    /// List available [Anthropic] models.
    pub async fn list_models(&self) -> Result<Vec<Model>> {
        let res = self.0.get(format!("{API_ENDPOINT}/models?limit=1000"))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: AnthropicError = from_value(error.clone())?;
            return Err(err.into());
        }

        Ok(from_value(json["data"].clone())?)
    }

    /// Get information about a specific model.
    pub async fn get_model(&self, model_id: &str) -> Result<Model> {
        let res = self.0.get(format!("{API_ENDPOINT}/models/{model_id}"))
            .send().await?;

        let json: Value = res.json().await?;
        if let Some(error) = json.get("error") {
            let err: AnthropicError = from_value(error.clone())?;
            return Err(err.into());
        }

        Ok(from_value(json)?)
    }
}

#[cfg(test)]
mod tests {
    use crate as renfield;
    use crate::client::anthropic::*;
    use once_cell::sync::Lazy;

    static CLIENT: Lazy<Anthropic> = Lazy::new(|| {
        dotenv::dotenv().ok();
        let token = std::env::var("ANTHROPIC_API_KEY")
            .expect("missing ANTHROPIC_API_KEY");

        Anthropic::from_token(&token)
    });

    #[tokio_shared_rt::test(shared)]
    async fn test_api_error() {
        let err = Anthropic::from_token("bad-fake-token")
            .list_models().await
            .unwrap_err();

        match err {
            crate::Error::Client(err) => match err {
                crate::client::Error::Anthropic(err) => {
                    assert_eq!(err.message, "invalid x-api-key");
                    assert_eq!(err.kind, "authentication_error");
                },
                _ => panic!("got error from other client"),
            },
            _ => panic!("got incorrect error type"),
        }
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate() {
        let req = Request::from_model("claude-3-haiku-20240307")
            .push_message(msg!("user", "hello, who are you?"))
            .max_tokens(20);

        let res = CLIENT.generate(&req).await.unwrap();
        assert!(res.usage.output_tokens > 0);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_sampling_parameters() {
        let req = Request::from_model("claude-3-haiku-20240307")
            .push_message(msg!("user", "hello, who are you?"))
            .tool_choice(ToolChoice::auto(None))
            .temperature(1.5)
            .max_tokens(20)
            .top_p(0.2)
            .top_k(5);

        let res = CLIENT.generate(&req).await.unwrap();
        assert_ne!(res.usage.output_tokens, 0);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_image_input() {
        let url = "https://upload.wikimedia.org/wikipedia/commons/a/a7/Camponotus_flavomarginatus_ant.jpg";
        let req = Request::from_model("claude-3-haiku-20240307")
            .push_message(msg!("user", "describe this image", Image::url(url)))
            .max_tokens(20);

        let res = CLIENT.generate(&req).await.unwrap();
        assert_ne!(res.usage.output_tokens, 0);

        let mut has_text = false;
        match res.content {
            Content::Parts(parts) => {
                for part in parts {
                    if matches!(part, ContentPart::Text(_)) {
                        has_text = true;
                        break;
                    }
                }
            },
            Content::Text(_) => { has_text = true; }
        }

        assert!(has_text);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_document_input() {
        use base64::Engine;
        use base64::prelude::BASE64_STANDARD;

        // Downloads a PDF and feeds it to the model as base64
        // inline media bytes.
        let url = "https://www.melbpc.org.au/wp-content/uploads/2017/10/small-example-pdf-file.pdf";
        let bytes = reqwest::get(url).await.unwrap()
            .bytes().await.unwrap();

        let base64 = BASE64_STANDARD.encode(bytes);
        let req = Request::from_model("claude-haiku-4-5")
            .push_message(msg!("user", Document::base64(&base64, MediaType::Pdf)))
            .max_tokens(20);

        let res = CLIENT.generate(&req).await.unwrap();
        assert_ne!(res.usage.output_tokens, 0);

        // Try sending the file as a URL.
        let req = Request::from_model("claude-haiku-4-5")
            .push_message(msg!("user", Document::pdf_url(url)))
            .max_tokens(20);

        let res = CLIENT.generate(&req).await.unwrap();
        assert_ne!(res.usage.output_tokens, 0);

        // Try sending plain text as a document.
        let req = Request::from_model("claude-haiku-4-5")
            .push_message(msg!("user", Document::text("hello, world", MediaType::Text)))
            .max_tokens(20);

        let res = CLIENT.generate(&req).await.unwrap();
        assert_ne!(res.usage.output_tokens, 0);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_search_results() {
        todo!();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_thinking() {
        let req = Request::from_model("claude-haiku-4-5")
            .push_message(msg!("user", "what is 5+7?"))
            .thinking(ThinkingConfig::enabled(1024))
            .max_tokens(1042);

        let res = CLIENT.generate(&req).await.unwrap();
        let mut has_thinking = false;
        match res.content {
            Content::Parts(parts) => {
                for part in parts {
                    if matches!(part, ContentPart::Thinking(_)) {
                        has_thinking = true;
                    }
                }
            },
            _ => panic!("no thinking returned"),
        }

        assert!(has_thinking);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_tool_use() {
        #![allow(dead_code)]
        use crate::tools::{Tool, tool, schema};

        #[derive(tool, schema)]
        #[tool(rename = "get_weather")]
        #[tool(desc = "get the weather at a specific location")]
        struct GetWeather {
            /// The city name followed by country, e.g. Paris, France.
            pub location: String,
        }

        let req = Request::from_model("claude-3-haiku-20240307")
            .push_message(msg!("user", "what's the weather in Paris, France?"))
            .push_tool(GetWeather::as_anthropic_tool())
            .max_tokens(50);

        let res = CLIENT.generate(&req).await.unwrap();
        assert_ne!(res.usage.output_tokens, 0);

        let mut has_tool_call = false;
        match res.content {
            Content::Parts(parts) => {
                for part in parts {
                    match part {
                        ContentPart::ToolUse(tool) if tool.name == "get_weather" => {
                            has_tool_call = true;
                        },
                        _ => continue,
                    }
                }
            },
            Content::Text(_) => panic!("no tool call returned"),
        }

        assert!(has_tool_call);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_web_search_tool() {
        let req = Request::from_model("claude-3-5-haiku-latest")
            .push_message(msg!("user", "what is the weather in NYC?"))
            .push_tool(serde_json::json!({
                "type": "web_search_20250305",
                "name": "web_search",
                "max_uses": 1,
            }))
            .max_tokens(50);

        let res = CLIENT.generate(&req).await.unwrap();
        assert_ne!(res.usage.output_tokens, 0);

        let mut has_server_tool_call = false;
        let mut has_search_results = false;
        if let Content::Parts(parts) = res.content {
            for part in parts {
                match part {
                    ContentPart::ServerToolUse(tool_use) => {
                        if tool_use.name == ServerTool::WebSearch {
                            has_server_tool_call = true;
                        }
                    },
                    ContentPart::WebSearchToolResult(_) => {
                        has_search_results = true;
                    },
                    _ => continue,
                }
            }
        }

        assert!(has_search_results);
        assert!(has_server_tool_call);
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_web_fetch_tool() {
        let req = Request::from_model("claude-3-5-haiku-latest")
            .push_message(msg!("user", "analyze this web page: https://rust-lang.org/"))
            .push_tool(serde_json::json!({
                "type": "web_fetch_20250910",
                "name": "web_fetch",
                "max_uses": 1,
            }))
            .max_tokens(100);

        let res = CLIENT.generate(&req).await.unwrap();
        println!("{res:?}");
        assert_ne!(res.usage.output_tokens, 0);

        let mut has_server_tool = false;
        if let Content::Parts(parts) = res.content {
            for part in parts {
                match part {
                    ContentPart::ServerToolUse(tool_use) => {
                        if tool_use.name == ServerTool::WebFetch {
                            has_server_tool = true;
                        }
                    }
                    _ => continue,
                }
            }
        }
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_code_execution() {
        todo!();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_bash_code_execution() {
        todo!();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_generate_mcp_tool_use() {
        // Test mcp tool results as well.
        todo!();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_stream() {
        todo!();
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_list_models() {
        let models = CLIENT.list_models().await.unwrap();
        assert!(!models.is_empty());
    }

    #[tokio_shared_rt::test(shared)]
    async fn test_get_model() {
        let models = CLIENT.list_models().await.unwrap();
        assert!(!models.is_empty());

        let model = CLIENT.get_model(&models[0].id).await.unwrap();
        assert_eq!(model.id, models[0].id);
    }
}
