//! [Ollama](https://ollama.com/) client.
//!
//! # Examples
//! ## List Available Models
//! ```no_run
//! use agent::ollama::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("http://localhost:11434");
//!     client.list_models().await.unwrap().iter()
//!         .for_each(|model| println!("{}", model.name));
//! }
//! ```
//! ## Message Completion
//! ```no_run
//! use agent::ollama::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("http://localhost:11434");
//!     let req = ChatRequest::from_prompt("gemma3n:e4b", "hello");
//!     let res = client.completion(req).await.unwrap();
//!     println!("{}", res.message.content.as_ref().unwrap());
//! }
//! ```
//! ## Message Completion (Streaming)
//! ```no_run
//! use agent::ollama::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("http://localhost:11434");
//!     let req = ChatRequest::from_prompt("gemma3n:e4b", "hello");
//!     let mut sse = client.stream_completion(req).await.unwrap();
//!     while let Some(event) = sse.next::<String>().await {
//!         match event {
//!             Ok(data) => println!("{data:?}"),
//!             Err(e) => panic!("stream error: {e}"),
//!         }
//!     }
//! }
//! ```

mod chat;
pub use chat::*;

mod client;
pub use client::*;

mod models;
pub use models::*;
