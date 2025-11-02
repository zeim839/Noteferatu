//! MCP host implementation.
//!
//! An MCP host acts as a container and coordinator for MCP server
//! connections. It creates and manages multiple client instances,
//! controls connection permissions and lifecycle, handles
//! authentication, and routes requests to clients.
//!
//! The [`Host`] implementation supports all MCP [transport
//! protocols](super::Transport) (i.e. `stdio`, `Streamable HTTP`, and
//! legacy `HTTP+SSE`).
//!
//! # Examples
//!
//! ## Connecting to servers
//!
//! ```
//! ```
//!
//! ## Using prompts, tools, and resources
//!
//! ```
//! ```
//!
//! ## Querying server metadata
//!
//! ```
//! ```
//!
//! ## Subscriptions & Notifications
//!
//! ```
//! ```
//!

mod host;
pub use host::*;
