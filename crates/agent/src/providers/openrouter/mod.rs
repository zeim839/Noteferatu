//! [OpenRouter](https://openrouter.ai/) client.
//!
//! # Examples
//! ## List Available Models
//! ```no_run
//! use agent::providers::openrouter::*;
//! use agent::core::Client as _;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-openrouter-key");
//!     let models = client.list_models().await.unwrap();
//!     models.iter().for_each(|model| println!("{}", model.display_name));
//! }
//! ```
//!
//! ## Chat Completion Request
//! ### Non-Streaming
//! ```no_run
//! use agent::providers::openrouter::*;
//! use agent::core::Client as _;
//!
//! #[tokio::main]
//! async fn main() {
//!     let model = "deepseek/deepseek-chat-v3-0324:free";
//!     let client = Client::new("my-openrouter-key");
//!     let req = Request::from_prompt(model, "Hello, deepseek!");
//!     let res = client.completion(req).await.unwrap();
//!     println!("Received response: {res:?}");
//! }
//!
//! ```
//! ### Streaming
//! ```no_run
//! use agent::providers::openrouter::*;
//! use agent::core::Client as _;
//!
//! #[tokio::main]
//! async fn main() {
//!     let model = "deepseek/deepseek-chat-v3-0324:free";
//!     let client = Client::new("my-openrouter-key");
//!     let req = Request::from_prompt(model, "Hello, deepseek!");
//!     client.stream_completion(req, |res| {
//!         println!("{res:?}");
//!     }).await.unwrap();
//! }
//! ```

mod client;
pub use client::*;

mod model;
pub use model::*;

mod request;
pub use request::*;

mod error;
pub use error::*;
