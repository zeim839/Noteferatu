//! [Anthropic](https://www.anthropic.com/) client.
//!
//! Implements an Anthropic API client, which provides access to the
//! Claude model series. Claude is a highly performant, trustworthy,
//! and intelligent AI platform built by Anthropic. Claude excels at
//! tasks involving language, reasoning, analysis, coding, and more.
//!
//! # Examples
//! ## List Available Models
//! ```no_run
//! use agent::anthropic::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let models = client.list_models().await.unwrap();
//!     models.iter().for_each(|model| println!("{}", model.display_name));
//! }
//! ```
//! ## Message Completion
//! ```no_run
//! use agent::anthropic::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!
//!     // Create a request from a single user prompt.
//!     let req = MessageRequest::from_prompt(
//!         "claude-3-haiku-20240307",
//!         "Hello, Claude!",
//!     );
//!
//!     // Or from a message history...
//!     let messages = vec![Message {
//!         role: Role::User,
//!         content: "Hello, Claude!".to_string(),
//!     }];
//!
//!     let req = MessageRequest::from_messages(
//!         "claude-3-haiku-20240307",
//!         messages,
//!     );
//!
//!     let res = client.messages(req).await.unwrap();
//!     let content = &res.content[0];
//!     if let ContentResponse::Text(res) = content {
//!         println!("Text response: {}", res.text);
//!     }
//! }
//! ```
//! ## Message Completion (Streaming)
//! ```no_run
//! use agent::anthropic::*;
//! use tokio_stream::StreamExt;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let req = MessageRequest::from_prompt(
//!         "claude-3-haiku-20240307",
//!         "Hello, Claude!",
//!     );
//!
//!     // Prints stream events.
//!     let mut stream = client.stream_messages(req).await.unwrap();
//!     while let Some(result) = stream.next().await {
//!         match result {
//!             Ok(res) => println!("{res:?}"),
//!             Err(e) => panic!("stream error: {e}"),
//!         }
//!     }
//! }
//! ```

mod client;
pub use client::*;

mod models;
pub use models::*;

mod messages;
pub use messages::*;
