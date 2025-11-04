//! Ollama API client.

mod client;
pub use client::*;

mod error;
pub use error::*;

mod ollama;
pub use ollama::*;

pub mod request;
pub use request::*;

pub mod response;
pub use response::*;

pub mod message;
pub use message::*;

mod model;
pub use model::*;
