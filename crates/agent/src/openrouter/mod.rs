//! [OpenRouter](https://openrouter.ai/) client.
//!
//! Implements an OpenRouter API client, which provides a unified API
//! for interfacing with hundreds of AI models through a single
//! endpoint.
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
//! ## Completion Request
//! ### Non-Streaming
//! ```no_run
//! use agent::openrouter::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-openrouter-key");
//!     let req = ChatRequest::from_prompt(
//!         "deepseek/deepseek-chat-v3-0324:free",
//!         "What is the meaning of life?",
//!     ).with_usage(true);
//!
//!     let res = client.completion(req).await.unwrap();
//!     let choices = res.choices.unwrap();
//!     println!("Response: {}", choices[0].text.clone().unwrap());
//!
//!     // Inspect usage metadata.
//!     let usage = res.usage.unwrap();
//!     println!("Total tokens billed: {}", usage.total_tokens);
//! }
//! ```
//! The OpenRouter completions route responds with the [ChatResponse]
//! object, which contains either a prompt response, message chain, or
//! finish reason.
//! ### Streaming
//! ## Chat Completion Request
//! ### Non-Streaming
//! ```no_run
//! use agent::openrouter::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-openrouter-key");
//!
//!     let messages = vec![
//!         Message { role: Role::User, content: "hello".to_string() }
//!     ];
//!
//!     let req = ChatRequest::from_messages(
//!         "deepseek/deepseek-chat-v3-0324:free",
//!         messages,
//!     ).with_usage(true);
//!
//!     let res = client.chat_completion(req).await.unwrap();
//!     let choices = res.choices.unwrap();
//!     println!("Response: {}", choices[0].message.as_ref().unwrap().content);
//!
//!     // Inspect usage metadata.
//!     let usage = res.usage.unwrap();
//!     println!("Total tokens billed: {}", usage.total_tokens);
//!
//! }
//! ```
//! ### Streaming

mod errors;
pub use errors::*;

mod client;
pub use client::*;

mod models;
pub use models::*;

mod chat;
pub use chat::*;

mod stream;
pub use stream::*;
