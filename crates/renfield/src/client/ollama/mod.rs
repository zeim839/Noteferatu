//! Ollama API client.
//!
//! Implements an Ollama API client supporting both local and remote
//! endpoints (e.g. Ollama cloud).
//!
//! # Examples
//!
//! ## Generate a chat completion
//!
//! ```
//! use renfield::client::ollama::{Ollama, Request, Message, msg};
//!
//! #[tokio::main]
//! async fn main() {
//!     let req = Request::from_model("qwen3:0.6b")
//!         .push_message(msg!("user", "hello, who are you?"))
//!         .num_predict(100) // maximum output tokens.
//!         .think(true); // enable reasoning.
//!
//!     let res = Ollama::new().generate(&req).await.unwrap();
//!     if let Message::Assistant { content, thinking, .. } = res.message {
//!         if let Some(thinking) = thinking {
//!             println!("Reasoning: {thinking}");
//!         }
//!         println!("Response: {content}");
//!     }
//! }
//! ```
//!
//! ## List Available Models
//!
//! ```
//! use renfield::client::ollama::Ollama;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Ollama::new();
//!     let mut models = client.list_models().await.unwrap();
//!     println!("Models: {models:?}");
//!
//!     // Get information on a specific model.
//!     if !models.is_empty() {
//!         let model_name = models.remove(0).name;
//!         let details = client.show_model_details(&model_name)
//!             .await.unwrap();
//!
//!         println!("{model_name} details: {details:?}");
//!     }
//! }
//! ```
//!
//! ## Tool Calling
//!
//! ```
//! use renfield::client::ollama::{Ollama, Request, Message, msg};
//! use renfield::tools::{Tool, tool, schema};
//!
//! // Define the tool's input arguments.
//!
//! /// Get the current weather at a specific location.
//! #[derive(tool, schema)]
//! #[tool(rename = "get_weather")]
//! struct GetWeather {
//!
//!     /// City name, followed by country (e.g. Paris, France).
//!     location: String,
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let req = Request::from_model("qwen3:0.6b")
//!         .push_message(msg!("user", "what is the weather in Paris, France?"))
//!         .push_tool(GetWeather::as_ollama_tool()) // generates tool schema.
//!         .think(true);
//!
//!     let res = Ollama::new().generate(&req).await.unwrap();
//!     if let Message::Assistant { tool_calls, .. } = res.message {
//!         let tool_calls = tool_calls.unwrap();
//!         for tool_call in tool_calls {
//!             println!("model called tool: {}", tool_call.function.name);
//!         }
//!     }
//! }
//! ```
//!
//! ## Streaming
//!
//! To stream the generated response, ensure that the
//! [`stream`](Request::stream) parameter is enabled.
//!
//! ```
//! use renfield::client::ollama::{Ollama, Request, Response, msg};
//!
//! #[tokio::main]
//! async fn main() {
//!     let req = Request::from_model("qwen3:0.6b")
//!         .push_message(msg!("user", "what is 5+7?"))
//!         .stream(true);
//!
//!     // Create a response channel.
//!     let (tx, mut rx) = tokio::sync::mpsc::channel::<Response>(20);
//!
//!     // Process the request in a separate thread.
//!     let handler = tokio::spawn(async move {
//!         Ollama::new().stream(&req, tx).await.unwrap();
//!     });
//!
//!     // Print responses as they arrive.
//!     while let Some(response) = rx.recv().await {
//!         println!("{response:?}");
//!     }
//!
//!     // Wait for the client thread to finish.
//!     handler.await.unwrap();
//! }
//! ```
//!
//! ## Using Ollama Cloud
//!
//! The [`Ollama::from_remote`] function creates an [Ollama] client
//! that connects to a remote endpoint with an optional authentication
//! bearer token.
//!
//! ```
//! use renfield::client::ollama::{Ollama, Request, Message, msg};
//!
//! #[tokio::main]
//! async fn main() {
//!
//!     // Load API token from environment variables.
//!     dotenv::dotenv().ok();
//!     let api_key = std::env::var("OLLAMA_API_KEY")
//!         .expect("missing Ollama cloud API key");
//!
//!     // Instantiate the remote client.
//!     let client = Ollama::from_remote("https://ollama.com", Some(&api_key));
//!
//!     // Create a simple request.
//!     let req = Request::from_model("gpt-oss:20b-cloud")
//!         .push_message(msg!("user", "hello, who are you?"));
//!
//!     // Generate the response.
//!     let res = client.generate(&req).await.unwrap();
//!
//!     // Handle the response message.
//!     if let Message::Assistant { content, thinking, .. } = res.message {
//!         if let Some(thinking) = thinking {
//!             println!("Reasoning: {}", thinking);
//!         }
//!         println!("Response: {}", content);
//!     }
//! }
//! ```

mod client;
pub use client::*;

mod error;
pub use error::*;

pub mod message;
pub use message::*;

pub mod model;
pub use model::*;

pub mod request;
pub use request::*;

pub mod response;
pub use response::*;

pub mod tools;
pub use tools::*;
