//! Generate tool schemas from custom types.
//!
//! Function calling (also known as tool calling) provides a powerful
//! and flexible way for LLM models to interface with external systems
//! and access data outside their training data.
//!
//! The [`tools`](Self) module lets you define standard JSON schemas
//! and LLM function calling schemas from arbitrary Rust structs.
//!
//! # Examples
//!
//! ## Deriving schemas from algebraic types
//!
//! Algebraic types are composite data types built by combining other
//! types using "and" (product) and "or" (sum) operations. In Rust,
//! structs correspond to the "and" (product) operation and enums
//! correspond to the "or" (sum) operation.
//!
//! To derive schemas from algebraic types means to generate schemas
//! from composite enum or struct types. In renfield, this is made
//! simple by the [`tool`] and [`schema`] derive macros.
//!
//! ```
//! use renfield::tools::{Tool, tool, Schema, schema};
//!
//! /// Units the temperature will be returned in.
//! #[derive(schema)]
//! #[schema(rename_all = "lowercase")]
//! enum TempUnits {
//!     Celsius,
//!     Fahrenheit,
//! }
//!
//! /// Retrieves the current weather for the given location.
//! #[derive(schema, tool)]
//! #[tool(rename = "get_weather")]
//! struct GetWeather {
//!     #[schema(desc = "City and country e.g. Bogotá, Colombia")]
//!     location: String,
//!     units: TempUnits,
//! }
//!
//! // Example: generate JSON schema for TempUnits.
//! assert_eq!(TempUnits::schema(), serde_json::json!({
//!     "enum": ["celsius", "fahrenheit"],
//!     "description": "Units the temperature will be returned in.",
//! }));
//!
//! // Example: generate an OpenAI tool schema for GetWeather.
//! assert_eq!(GetWeather::as_openai_tool(), serde_json::json!({
//!     "type": "function",
//!     "name": "get_weather",
//!     "description": "Retrieves the current weather for the given location.",
//!     "parameters": {
//!         "type": "object",
//!         "description": "Retrieves the current weather for the given location.",
//!         "properties": {
//!             "location": {
//!                 "type": "string",
//!                 "description": "City and country e.g. Bogotá, Colombia",
//!             },
//!             "units": {
//!                 "enum": ["celsius", "fahrenheit"],
//!                 "description": "Units the temperature will be returned in.",
//!             },
//!         },
//!         "required": ["location", "units"]
//!     },
//! }));
//! ```
//!
//! JSON schemas are defined according to the [JSON schema
//! specification](https://json-schema.org/specification). Additionally,
//! the [`tool`] derive macro supports generating tool schemas for
//! Anthropic, OpenAI, Gemini, OpenRouter, and Ollama clients.
//!
//! ## JSON schemas for primitive types
//!
//! The [tools](Self) crate is designed to integrate seamlessly with
//! [serde], meaning types implementing [Serialize](serde::Serialize)
//! or [Deserialize](serde::Deserialize) also implement
//! [`Schema`]. This includes all Rust [primitive
//! types](std::primitive), as well as select
//! [Atomic](core::sync::atomic), and stdlib types (e.g. `Vec`,
//! `Option`, etc.).
//!
//! A special case is the `Option<T>` type, which inherits `T`'s
//! schema but is implicitly marked as optional (i.e. the associated
//! field name does not appear in the `required` schema field).
//!
//! ```
//! use renfield::tools::Schema;
//!
//! assert_eq!(String::schema(), serde_json::json!({
//!     "type": "string",
//! }));
//!
//! assert_eq!(<Vec<String>>::schema(), serde_json::json!({
//!     "type": "array",
//!     "items": {
//!         "type": "string",
//!     },
//! }));
//!
//! assert_eq!(u8::schema(), serde_json::json!({
//!     "type": "integer",
//!     "minimum": u8::MIN,
//!     "maximum": u8::MAX,
//! }));
//! ```
//! # Caveats (TODOs)
//!
//! The [`tools`](Self) module is still an early prototype and as such
//! there are a number of caveats which we intend to address in the
//! future.
//!
//! * Lacks seamless integration with [serde]
//!   - Serde helper attributes are not natively recognized.
//!   - Certain serde attributes like `flatten` are not supported.
//! * JSON schema spec is not strictly implemented.
//!   - Improve support for OpenAI `strict` mode.
//!   - Implement strict `additionalProperties`, `items`, and `type`
//!     control for enum types.
//! * Fix interoperability issues between `Tool` and `Schema`
//!   - Setting a schema description may overwrite a tool description.
//!

mod schema;
pub use schema::*;

mod tool;
pub use tool::*;
