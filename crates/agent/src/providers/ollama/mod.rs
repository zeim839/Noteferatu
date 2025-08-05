//! [Ollama](https://ollama.com/) client.
//!
//! # Examples
//! ## List Available Models
//! ```no_run
//! use agent::providers::ollama::*;
//! use agent::core::Client as _;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("http://localhost:11434");
//!     client.list_models().await.unwrap().iter()
//!         .for_each(|model| println!("{}", model.display_name));
//! }
//! ```
//!
//! ## Message Completion
//! ```no_run
//! use agent::providers::ollama::*;
//! use agent::core::Client as _;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("http://localhost:11434");
//!     let req = Request::from_prompt("gemma3n:e4b", "hello");
//!     let res = client.completion(req).await.unwrap();
//!     println!("{res:?}");
//! }
//! ```
//!
//! ## Message Completion (Streaming)
//! ```no_run
//! use agent::providers::ollama::*;
//! use agent::core::Client as _;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("http://localhost:11434");
//!     let req = Request::from_prompt("gemma3n:e4b", "hello");
//!     client.stream_completion(req, |res| {
//!         println!("{res:?}");
//!     }).await.unwrap();
//! }
//! ```

mod client;
pub use client::*;

mod request;
pub use request::*;

mod response;
pub use response::*;

mod message;
pub use message::*;

mod model;
pub use model::*;
