//! OpenAI API client.
//!
//! Implements an OpenAI chat completions API client. Some model
//! metadata routes are supported as well (e.g. list models, get model
//! information, etc.).
//!
//! # Examples
//!
//! ## Create a Request
//!
//! The [`Request`] struct and [`msg`] macro were designed to make
//! writing requests a simple, clear, and streamlined process. Request
//! properties can be accumulated in functional style, while the
//! [`msg`] macro supports constructing conversation messages from
//! variadic inputs.
//!
//! ```
//! use renfield::client::openai::{OpenAI, Request, msg};
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv::dotenv().ok();
//!     let api_token = std::env::var("OPENAI_API_KEY").unwrap();
//!     let client = OpenAI::from_token(&api_token);
//!     let req = Request::from_model("gpt-4o-mini")
//!         .push_message(msg!("developer", "You are a helpful assistant"))
//!         .push_message(msg!("user", "Hello, who are you?"))
//!         .max_completion_tokens(100);
//!
//!     let res = client.chat_completion(&req).await.unwrap();
//!     assert!(res.choices.len() > 0);
//!     println!("{res:?}");
//! }
//! ```
//!
//! ## List Available Models
//!
//! ```
//! use renfield::client::openai::OpenAI;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv::dotenv().ok();
//!     let api_token = std::env::var("OPENAI_API_KEY").unwrap();
//!     let client = OpenAI::from_token(&api_token);
//!     let models = client.list_models().await.unwrap();
//!     for model in models {
//!         println!("{model:?}");
//!     }
//! }
//! ```
//!
//! ## Audio Generation
//!
//! Speech generation is supported by OpenAI's `got-4o-audio-preview`
//! model. Speech generation requests must specify `Audio` as a
//! supported output [modality](Modality) and include an
//! [`AudioConfig`] object.
//!
//! The [`AudioConfig`] struct specifies which voice and output
//! format the model should use when generating speech.
//!
//! ```
//! use renfield::client::openai::{OpenAI, Request, AudioConfig, Modality, msg};
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv::dotenv().ok();
//!     let api_token = std::env::var("OPENAI_API_KEY").unwrap();
//!     let client = OpenAI::from_token(&api_token);
//!     let req = Request::from_model("gpt-4o-audio-preview")
//!         // Allow audio output
//!         .append_modalities(&mut vec![Modality::Text, Modality::Audio])
//!         // Configure default audio voice and output format.
//!         .audio(AudioConfig::default())
//!         .push_message(msg!("user", "say hello!"))
//!         .max_completion_tokens(50);
//!
//!     let res = client.chat_completion(&req).await.unwrap();
//!     assert!(res.choices.len() > 0);
//!     println!("{:?}", res.choices[0].clone());
//! }
//! ```
//!
//! ## Streaming
//!
//! ```
//! use renfield::client::openai::{OpenAI, Request, Response, msg};
//! use tokio::sync::mpsc::channel;
//!
//! #[tokio::main]
//! async fn main() {
//!     dotenv::dotenv().ok();
//!     let api_token = std::env::var("OPENAI_API_KEY").unwrap();
//!     let client = OpenAI::from_token(&api_token);
//!     let req = Request::from_model("gpt-4o-mini")
//!         .push_message(msg!("user", "hello, world!"))
//!         .stream(true);
//!
//!     let (tx, mut rx) = channel::<Response>(16);
//!     let handle = tokio::spawn(async move {
//!         client.stream_chat_completion(&req, tx).await.unwrap();
//!     });
//!
//!     // Print each chunk as it arrives.
//!     while let Some(res) = rx.recv().await {
//!         println!("{res:?}");
//!     }
//!
//!     assert!(handle.await.is_ok());
//! }
//! ```

mod client;
pub use client::*;

mod error;
pub use error::*;

pub mod message;
pub use message::*;

mod model;
pub use model::*;

mod openai;
pub use openai::*;

pub mod request;
pub use request::*;

pub mod response;
pub use response::*;
