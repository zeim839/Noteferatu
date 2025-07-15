//! [Ollama](https://ollama.com/) client.

mod chat;
pub use chat::*;

mod client;
pub use client::*;

mod errors;
pub use errors::*;

mod models;
pub use models::*;

mod stream;
pub use stream::*;
