//! [OpenAI](https://openai.com/) client.
//!
//! # Examples
//! ## List Models
//! ```no_run
//! use agent::providers::openai::*;
//! use agent::core::Client as _;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let models = client.list_models().await.unwrap();
//!     models.iter().for_each(|model| println!("{}", model.id));
//! }
//!
//! ```
//! ## Chat Completion
//! ```no_run
//! use agent::providers::openai::*;
//! use agent::core::Client as _;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let req = Request::from_prompt(
//!         "gpt-4.1-mini", "Hello, ChatGPT!"
//!     );
//!
//!     let res = client.completion(req).await.unwrap();
//!     println!("{res:?}");
//! }
//! ```
//! ## Chat Completion (Stream)
//! ```no_run
//! use agent::providers::openai::*;
//! use agent::core::Client as _;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let req = Request::from_prompt(
//!         "gpt-4.1-mini", "Hello, ChatGPT!"
//!     );
//!
//!     client.stream_completion(req, |res| {
//!         println!("{res:?}");
//!     }).await.unwrap();
//! }

mod client;
pub use client::*;

mod request;
pub use request::*;

mod response;
pub use response::*;

mod message;
pub use message::*;

mod error;
pub use error::*;

mod model;
pub use model::*;
