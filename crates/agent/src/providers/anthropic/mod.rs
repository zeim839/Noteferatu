//! [Anthropic](https://www.anthropic.com/) client.
//!
//! # Examples
//! ## List Available Models
//! ```no_run
//! use agent::providers::anthropic::*;
//! use agent::core::Client as _;
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
//! use agent::providers::anthropic::*;
//! use agent::core::Client as _;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!
//!     // Create a request from a single user prompt.
//!     let req = Request::from_prompt(
//!         "claude-3-haiku-20240307",
//!         "Hello, Claude!",
//!     );
//!
//!     let res = client.completion(req).await.unwrap();
//!     println!("{res:?}");
//! }
//! ```
//! ## Message Completion (Streaming)
//! ```no_run
//! use agent::providers::anthropic::*;
//! use agent::core::Client as _;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let req = Request::from_prompt(
//!         "claude-3-haiku-20240307",
//!         "Hello, Claude!",
//!     );
//!
//!     // Prints stream events.
//!     client.stream_completion(req, |res| {
//!         println!("{res:?}");
//!     }).await.unwrap();
//! }
//! ```

mod client;
pub use client::*;

mod error;
pub use error::*;

mod request;
pub use request::*;

mod stream;
pub use stream::*;

mod response;
pub use response::*;

mod message;
pub use message::*;

mod model;
pub use model::*;
