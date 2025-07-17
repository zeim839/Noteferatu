//! [Google AI](https://ai.google.dev/) client.
//!
//! Implements a Google AI client for accessing Google's Gemini models.
//! Gemini is Google's most capable AI model, designed to be multimodal
//! and handle text, images, and other data types.
//!
//! # Examples
//! ## List Models
//! ```no_run
//! use agent::google::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let models = client.list_models().await.unwrap();
//!     models.iter().for_each(|model| println!("{}", model.display_name));
//! }
//! ```
//! ## Chat Completion
//! ```no_run
//! use agent::google::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let req = ChatRequest::from_prompt("Hello, Gemini!");
//!
//!     let res = client.completions("gemini-1.5-flash", req).await.unwrap();
//!     let msg = &res.candidates[0].content;
//!     println!("{}", msg.parts[0].data.text.as_ref().unwrap());
//! }
//! ```
//! ## Chat Completion (Stream)
//! ```no_run
//! use agent::google::*;
//! use tokio_stream::StreamExt;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let req = ChatRequest::from_prompt("Hello, Gemini!");
//!
//!     let mut stream = client.stream_completions("gemini-1.5-flash", req).await.unwrap();
//!     while let Some(result) = stream.next().await {
//!         match result {
//!             Ok(res) => {
//!                 for candidate in res.candidates {
//!                     for part in candidate.content.parts {
//!                         if let Some(text) = part.data.text {
//!                             print!("{}", text);
//!                         }
//!                     }
//!                 }
//!             },
//!             Err(e) => panic!("stream error: {e}"),
//!         }
//!     }
//! }

mod models;
pub use models::*;

mod client;
pub use client::*;

mod chat;
pub use chat::*;
