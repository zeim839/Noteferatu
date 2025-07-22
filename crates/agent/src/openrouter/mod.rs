//! [OpenRouter](https://openrouter.ai/) client.
//!
//! # Examples
//! ## List Available Models
//! ```no_run
//! use agent::openrouter::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-openrouter-key");
//!     let models = client.list_models().await.unwrap();
//!     models.iter().for_each(|model| println!("{}", model.name));
//! }
//! ```
//! Calling [Client::list_models] will fetch models currently
//! available on OpenRouter, returning a vector of [Model].
//!
//! ## Chat Completion Request
//! ### Non-Streaming
//! ```no_run
//! use agent::openrouter::*;
//! use agent::openai::ChatRequest;
//!
//! #[tokio::main]
//! async fn main() {
//!     let model = "deepseek/deepseek-chat-v3-0324:free";
//!     let client = Client::new("my-openrouter-key");
//!     let req = ChatRequest::from_prompt(model, "Hello, deepseek!");
//!     let res = client.completion(req).await.unwrap();
//!     println!("Received response: {res:?}");
//! }
//!
//! ```
//! ### Streaming
//! ```no_run
//! use agent::openrouter::*;
//! use agent::openai::{ChatRequest, ErrorAPI};
//!
//! #[tokio::main]
//! async fn main() {
//!     let model = "deepseek/deepseek-chat-v3-0324:free";
//!     let client = Client::new("my-openrouter-key");
//!     let req = ChatRequest::from_prompt(model, "Hello, deepseek!");
//!
//!     let mut sse = client.stream_completion(req).await.unwrap();
//!     while let Some(event) = sse.next::<ErrorAPI>().await {
//!         match event {
//!             Ok(response) => println!("{response:?}"),
//!             Err(e) => panic!("Stream error: {e}"),
//!         }
//!     }
//! }
//! ```

mod client;
pub use client::*;

mod models;
pub use models::*;
