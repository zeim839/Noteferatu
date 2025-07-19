//! [Google AI](https://ai.google.dev/) client.
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
//!     let res = client.completion("gemini-1.5-flash", req).await.unwrap();
//!     let msg = &res.candidates[0].content;
//!     println!("{}", msg.parts[0].data.text.as_ref().unwrap());
//! }
//! ```
//! ## Chat Completion (Stream)
//! ```no_run
//! use agent::google::*;
//! use agent::openai::OpenAIError;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let req = ChatRequest::from_prompt("Hello, Gemini!");
//!
//!     let mut sse = client.stream_completion("gemini-1.5-flash", req).await.unwrap();
//!     while let Some(result) = sse.next::<OpenAIError>().await {
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
