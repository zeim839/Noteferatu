//! [Ollama](https://ollama.com/) client.

mod chat;
pub use chat::*;

mod client;
pub use client::*;

mod models;
pub use models::*;
