//! Build procedural MCP servers.
//!
//! An MCP server is an external service that provides large language
//! models (LLMs) with access to tools, data, and capabilities that
//! they cannot access on their own.
//!
//! The [server](Self) module enables developers to quickly build &
//! deploy MCP servers using a convenient, high-level Rust API.
//!
//! # Examples
//!
//! ## Prompts, resources, tools
//!
//! ```
//! use renfield::mcp::server::{ServerOptions, handler};
//! use renfield::tools::{tool, tool_info};
//! use renfield::mcp::protocol::{
//!     ToolRequest, ToolResponse, prompt, prompt_info,
//! };
//!
//! /// Get today's horoscope for an astrological sign.
//! #[derive(tool)]
//! #[tool_info(name = "get_horoscope")]
//! struct GetHoroscope {
//!     /// An astrological sign like Taurus or Aquarius.
//!     pub sign: String,
//! }
//!
//! // Defines a function to handle calls to the tool.
//! #[handler(tool = "GetHoroscope")]
//! fn handle_get_horoscope(input: ToolRequest<GetHoroscope>) -> ToolResponse<String> {
//!     Ok(format!("{}: You will befriend a baby otter.", input.sign))
//! }
//!
//! /// Asks the LLM to analyze code quality and suggest improvements.
//! #[derive(prompt)]
//! #[prompt_info(name = "code_review")]
//! struct CodeReview {
//!     /// The code to review.
//!     pub code: String,
//! }
//!
//! // Defines a function to handle requests to the prompt.
//! #[handler(prompt = "CodeReview")]
//! fn handle_code_review(_input: PromptRequest<CodeReview>) -> PromptResponse<String> {
//!     Ok("Please review this code...")
//! }
//!
//! // Defines a function to handle requests to a resource.
//! #[handler(resource = "file:///project/src/main.rs")]
//! fn handle_resource(_input: ResourceRequest) -> ResourceResponse {
//!     ResourceResponse::Text("fn main() {\n    println!(\"Hello world!\");\n}")
//! }
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     ServerOptions::empty_context()
//!         .handlers(vec![
//!             handle_get_horoscope,
//!             handle_code_review,
//!             handle_resource,
//!         ])
//!         .http_streamable("localhost:9919/mcp")
//!         .start().await
//! }
//! ```
//!
//! ## Stateful Interactions
//!
//! The [`Server`] struct is generically defined; it supports
//! "context" structs that pass modifiable state to handler functions,
//! enabling stateful interactions. In addition, the [`Context`]
//! struct can be used to emit notifications, progress updates, etc.
//!
//! ```
//! use renfield::mcp::server::{ServerOptions, Context, handler};
//! use renfield::mcp::protocol::{ToolRequest, ToolResponse};
//! use renfield::tools::{tool, tool_info};
//!
//! use std::sync::{Arc, Mutex};
//! use serde_json::{Value, json};
//!
//! // Define custom context (must be Send+Sync).
//! struct MyContext {
//!     counter: Arc<Mutex<u32>>,
//! }
//!
//! impl MyContext {
//!     fn new() -> Self {
//!         Self { counter: Arc::new(Mutex::new(0)) }
//!     }
//!
//!     fn increment(&self) {
//!         let mut count = self.counter.lock().unwrap();
//!         *count += 1;
//!     }
//! }
//!
//! /// Retrieves the current weather for the given location.
//! #[derive(tool)]
//! #[tool_info(name = "get_weather")]
//! struct GetWeather {
//!
//!     /// City and country e.g. Bogotá, Colombia.
//!     pub location: String,
//! }
//!
//! #[handler(tool = "GetWeather")]
//! async fn handle_get_weather(
//!     ctx: &Context<MyContext>,
//!     input: ToolRequest<GetWeather>
//! ) -> ToolResponse<Value> {
//!
//!     // Increment counter.
//!     ctx.state.increment();
//!
//!     // Emit notification.
//!     ctx.emit_notification("method_name", json!({
//!         "data": "some data here...",
//!     })).await?;
//!
//!     ToolResponse::Success(json!({
//!         "location": input.location,
//!         "temperature": 23.0,
//!     }))
//! }
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let context = MyContext::new();
//!     ServerOptions::from_context(&context)
//!         .handlers(vec![handle_get_weather])
//!         .http_streamable("localhost:9919/mcp")
//!         .start().await
//! }
//! ```
//!
//! ## Multi-resource handler
//!
//! When defining handlers for **resources** or **resource
//! templates**, the [`handler`] macro supports passing a function
//! that defines the items that the function is intended to handle.
//!
//! Since the definition of resource and resource templates is
//! non-generic, this is convenient in situations where defining a
//! separate handler for each resource is cumbersome.
//!
//! A caveat is that the mapping function is evaluated only at server
//! initialization (i.e. statically). Changes to resources must be
//! registered using the server's
//! [update_resources](Server::update_resources) function.
//!
//! ```
//! use renfield::mcp::server::{ServerOptions, handler};
//! use renfield::mcp::protocol::Resource;
//!
//! // Defines a set of resources for `handle_resources`.
//! fn define_resources() -> Vec<Resource> {
//!     vec![Resource {
//!         uri: "file:///project/src/main.rs",
//!         name: Some("main.rs"),
//!         title: Some("Rust Software Application Main File"),
//!         description: Some("Primary application entry point"),
//!         mime_type: Some("text/x-rust"),
//!         annotations: None,
//!     }]
//! }
//!
//! // Defines the resource handler.
//! // Notice `resources`, instead of the singular `resource`.
//! #[handler(resources = "define_resources")]
//! fn handle_resources(_input: ResourceRequest) -> ResourceResponse {
//!     ResourceResponse::Text("some text resource...")
//! }
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     ServerOptions::empty_context()
//!         .handlers(vec![handle_resources])
//!         .http_streamable("localhost:9919/mcp")
//!         .start().await
//! }
//! ```

mod server;
pub use server::*;

mod context;
pub use context::*;

mod options;
pub use options::*;

mod macros;
pub use macros::*;
