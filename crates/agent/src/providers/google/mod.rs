//! [Google AI](https://ai.google.dev/) client.
//!
//! # Examples
//! ## List Models
//! ```no_run
//! use agent::providers::google::*;
//! use agent::core::Client as _;
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
//! use agent::providers::google::*;
//! use agent::core::Client as _;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let req = Request::from_prompt("gemini-2.5-pro", "Hello, Gemini!");
//!
//!     let res = client.completion(req).await.unwrap();
//!     println!("{res:?}");
//! }
//! ```
//! ## Chat Completion (Stream)
//! ```no_run
//! use agent::providers::google::*;
//! use agent::core::Client as _;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let req = Request::from_prompt("gemini-2.5-pro", "Hello, Gemini!");
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

mod content;
pub use content::*;

mod error;
pub use error::*;

mod model;
pub use model::*;
