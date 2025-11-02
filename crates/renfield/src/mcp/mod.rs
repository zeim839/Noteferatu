//! Model Context Protocol (MCP) clients & servers.
//!
//! The [mcp](Self) module is [FastMCP](https://gofastmcp.com/) for
//! Rust. It provides developers with a plug-and-play MCP host
//! implementation, protocol type primitives, and a high-level Rust
//! API for building MCP servers (supporting HTTP, SSE, and stdio
//! transport protocols).
//!
//! [Hosts](host) are defined according to the [MCP
//! spec](https://modelcontextprotocol.io/specification/2025-06-18/architecture#host):
//! they manage and coordinate one or more MCP clients, each of which
//! handles a single connection to an MCP server.
//!
//! [Servers](server) provide LLMs with specialized context and
//! capabilities, such as exposing external resources, tools, and
//! prompt. The [mcp](Self) module exposes procedural macros, allowing
//! developers to quickly build and deploy MCP servers using a
//! convenient high-level Rust API.
//!
//! # Features
//! * [`host`] module
//!   - Manage one or more connections to MCP servers.
//!   - Supports standard client features (roots, elicitation, sampling).
//!   - Supports `StreamableHTTP`, `HTTP+SSE` (legacy), and `stdio` transport protocols.
//!   - Bidirectional subscriptions & notification streams.
//!   - Client authentication.
//! * [`server`] module
//!   - Develop & host MCP servers.
//!   - Procedural macros (in the style of [Actix Web Server](https://actix.rs/))
//!   - Supports `StreamableHTTP`, `HTTP+SSE` (legacy), and `stdio` transport protocols.
//!   - Authentication middleware.
//!
//! # Examples
//!
//! ## Interacting with MCP servers
//!
//! ```
//! use renfield::mcp::host::{Host, conn_uri};
//! use renfield::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!
//!     // Instantiate MCP host.
//!     let mut host = Host::default();
//!
//!     // Connect to StreamableHTTP server.
//!     host.connect("DeepWikiMCP", conn_uri!("mcp", "https://mcp.deepwiki.com/mcp")).await?;
//!
//!     // Connect to legacy HTTP+SSE server.
//!     host.connect("DeepWikiSSE", conn_uri!("sse", "https://mcp.deepwiki.com/sse")).await?;
//!
//!     // Query available tools.
//!     println!("Available tools: {}", host.list_tools().await?);
//!
//!     // Query available resources.
//!     println!("Available resources: {}", host.list_resources().await?);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Hosting an MCP server
//!
//! ```
//! use renfield::mcp::server::{ServerOptions, handler};
//! use renfield::mcp::protocol::{ToolRequest, ToolResponse};
//! use renfield::tools::{tool, tool_info};
//!
//! use serde_json::{Value, json};
//!
//! /// Retrieves the current weather for the given location.
//! #[derive(tool)]
//! #[tool_info(name = "get_weather")]
//! pub struct GetWeather {
//!
//!     /// City and country e.g. Bogotá, Colombia.
//!     pub location: String,
//! }
//!
//! #[handler(tool = "GetWeather")]
//! pub async fn handle_get_weather(input: ToolRequest<GetWeather>) -> ToolResponse<Value> {
//!     ToolResponse::Success(json!({
//!         "location": input.location,
//!         "temperature": 23.0,
//!     }))
//! }
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     ServerOptions::empty_context()
//!         .handlers(vec![handle_get_weather])
//!         .http_streamable("localhost:9919/mcp")
//!         .start().await
//! }
//! ```

pub mod host;
pub mod server;

pub mod protocol;
pub use protocol::*;
