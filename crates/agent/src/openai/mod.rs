//! [OpenAI](https://openai.com/) client.
//!
//! With the OpenAI API, you can use a large language model to
//! generate text from a prompt, as you might using ChatGPT.
//!
//! # Examples
//! ## List Models
//! ```no_run
//! use agent::openai::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let models = client.list_models().await.unwrap();
//!     models.iter().for_each(|model| println!("{}", model.id));
//! }
//! ```
//! ## Chat Completion
//! ```no_run
//! use agent::openai::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let req = ChatRequest::from_prompt(
//!         "gpt-4.1-mini", "Hello, ChatGPT!"
//!     );
//!
//!     let res = client.completion(req).await.unwrap();
//!     let msg = &res.choices[0].message.as_ref().unwrap();
//!     println!("{}", msg.content.as_ref().unwrap());
//! }
//! ```
//! ## Chat Completion (Stream)
//! ```no_run
//! use agent::openai::*;
//! use tokio_stream::StreamExt;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("my-api-key");
//!     let req = ChatRequest::from_prompt(
//!         "gpt-4.1-mini", "Hello, ChatGPT!"
//!     );
//!
//!     let mut stream = client.stream_completion(req).await.unwrap();
//!     while let Some(result) = stream.next().await {
//!         match result {
//!             Ok(res) => {
//!                 // Do something with response...
//!             },
//!             Err(e) => panic!("stream error: {e}"),
//!         }
//!     }
//! }

mod models;
pub use models::*;

mod client;
pub use client::*;

mod errors;
pub use errors::*;

mod chat;
pub use chat::*;

mod stream;
pub use stream::*;
