//! LLM client implementations.
//!
//! This module implements API clients for OpenAI, Anthropic,
//! OpenRouter, Google Gemini, and Ollama. It supports multiple
//! input/output modalities (text, audio, file, image, etc.),
//! streaming and static generations, tool calling, and more.
//!
//! Additionally, the [`Client`] trait (together with standardized
//! [Request] and [Response] implementations) provides a unified,
//! extensible API for interacting with LLM clients, abstracting away
//! implementation differences across API providers.
//!
//! # Getting Started
//!
//! * See the individual client implementations if you intend to
//!   interact with **only a single API**.
//!   - E.g. [anthropic], [gemini], [openai], [openrouter], [ollama].
//! * See the [Examples](#examples) section to get started with the
//!   **unified** [`Client`] API.
//!   - The [Request] documentation shows you how to build client
//!     requests in functional programming style.
//!   - The [Response] documentation explains how to read client responses.
//!
//! # Features
//!
//! * Chat completion clients for OpenAI, Anthropic, OpenRouter,
//!   Google Gemini, and Ollama.
//! * Streaming & static responses.
//! * First-in-class tool calling support using
//!   [tools](renfield::tools) module.
//! * Provider-dependent APIs as well as a unified API under the
//!   [Client] trait.
//! * Audio, video, text, file input/output (depending on model support).
//! * Generic [Request] and [Response] types that are recognized by
//!   all clients.
//!
//! # Examples
//!
//! ## Generating Text
//!
//! ```
//! use renfield::client::anthropic::Anthropic;
//! use renfield::client::{Client, Request, Role, msg};
//!
//! #[tokio::main]
//! async fn main() -> renfield::Result<()> {
//!     let api_token = std::env::var("ANTHROPIC_API_KEY").unwrap();
//!     let client = Anthropic::from_token(&api_token);
//!     let req = Request::from_model("claude-sonnet-4-5")
//!         .system_prompt("You are a helpful assistant")
//!         .push_message(msg!(Role::User, "Hello, what is your name?"))
//!         .max_tokens(50);
//!
//!     let res = client.generate(&req.into()).await?;
//!     assert!(res.choices.len() > 0);
//!     println!("{res}");
//!     Ok(())
//! }
//!
//! ```
//!
//! ## Streaming
//!
//! Streaming allows you to read a client's response as it is being
//! generated in real-time. All [Client] implementations support
//! streaming.
//!
//! ```
//! use renfield::client::{Client, Request, Response, Role, msg};
//! use renfield::client::ollama::Ollama;
//!
//! #[tokio::main]
//! async fn main() -> renfield::Result<()> {
//!
//!     // Spawn a separate thread to handle the request.
//!     let (tx, rx) = std::sync::mpsc::channel::<Response>();
//!     tokio::spawn(async move {
//!         let req = Request::from_model("qwen3:0.6b")
//!             .push_message(msg!(Role::User, "Hello, what is your name?"))
//!             .max_tokens(50);
//!
//!         Ollama::default().generate(&req, tx).await.unwrap();
//!     });
//!
//!     // Print responses as they arrive.
//!     while let Ok(response) = rx.recv() {
//!         println!("{response}");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Generating Speech
//!
//! ```
//! use renfield::client::openai::{OpenAI, VoiceConfig};
//! use renfield::client::{Request, Role, msg};
//!
//! #[tokio::main]
//! async fn main() -> renfield::Result<()> {
//!     todo!();
//! }
//! ```
//!
//! ## Files, audio, and image inputs
//!
//! ```
//! ```
//!
//! ## Fetching supported models
//!
//! ```
//! use renfield::client::openrouter::OpenRouter;
//! use renfield::client::Client;
//!
//! #[tokio::main]
//! async fn main() -> renfield::Result<()> {
//!     let api_token = std::env::var("OPENROUTER_API_KEY").unwrap();
//!     let client = OpenRouter::from_token(&api_token);
//!     let models = client.list_models().await?;
//!     assert!(models.len() > 0);
//!     println!("{models}");
//!     Ok(())
//! }
//! ```
//!
//! ## Tool Calling
//!
//! ```
//! use renfield::client::gemini::Gemini;
//! use renfield::client::{Client, Request, Role, msg};
//! use renfield::tools::{tool, tool_info};
//!
//! /// Get the current weather at a specific location.
//! #[derive(tool)]
//! #[tool_info(name = "get_weather")]
//! struct GetWeather {
//!     /// City followed by country, e.g. Bogota, Colombia.
//!     pub location: String,
//! }
//!
//! #[tokio::main]
//! async fn main() -> renfield::Result<()> {
//!     let api_token = std::env::var("GEMINI_API_KEY").unwrap();
//!     let client = Gemini::from_token(&api_token);
//!     let req = Request::from_model("gemini-2.5-flash")
//!         .push_message(msg!(Role::User, "what is the current weather?"))
//!         .tools(vec![GetWeather::gemini_tool_spec()])
//!         .require_tool_use();
//!
//!     let res = client.generate(&req).await?;
//!     assert!(res.choices.len() > 0);
//!     println!("{res}");
//!     Ok(())
//! }
//! ```

pub mod anthropic;

#[allow(clippy::module_inception)]
mod client;
pub use client::*;

mod error;
pub use error::*;

pub mod gemini;

mod model;
pub use model::*;

pub mod ollama;
pub mod openai;
pub mod openrouter;

mod request;
pub use request::*;

mod response;
pub use response::*;
