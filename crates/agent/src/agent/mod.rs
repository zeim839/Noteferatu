//! [Client](crate::core::Client) for interfacing with multiple LLM
//! providers.
//!
//! Implements an [SQLite](database) backend for automatically
//! recording chat completion [requests](crate::core::Request) and
//! [responses](crate::core::Response).

mod agent;
pub use agent::*;

mod manager;
pub use manager::*;

mod conversation;
pub use conversation::*;

mod context;
pub use context::*;

mod schema;
pub use schema::*;
