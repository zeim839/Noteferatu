//! [Anthropic](https://www.anthropic.com/) client.
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
//!     let res = client.completion(req).await.unwrap();
//!     let content = &res.content[0];
//!     println!("{}", content.text.as_ref().unwrap());
//! }
//! ```
//! ## Message Completion (Streaming)
//! ```no_run
//! use agent::anthropic::*;
//! use agent::openai::ErrorAPI;
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
//!     let mut stream = client.stream_completion(req).await.unwrap();
//!     while let Some(result) = stream.next::<ErrorAPI>().await {
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
