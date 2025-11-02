/// Derive the [`Tool`] trait for an arbitrary Rust type.
///
/// Generates a [`Tool`] implementation for an algebraic type
/// (e.g. `enum` or `struct`).
///
/// <div class="warning">
/// The tool derive macro assumes the type implements the Schema
/// trait.
/// </div>
///
/// # The `tool` helper attribute
///
/// The `tool` helper attribute supports the following options:
/// * `#[tool(rename = "...")]`: Override the default tool name.
/// * `#[tool(desc = "...")]`: Set a custom tool description.
///
/// Additionally, docstrings over a tool definition may serve as tool
/// descriptions (i.e. they work the same as `#[tool(desc =
/// "...")]`. If both a docstring and an explicit `desc` value are
/// set, the explicit `desc` description takes precedence.
///
/// ## Example
///
/// ```
/// use renfield::tools::{tool, schema};
///
/// // Both `tool` and `schema` must be derived!
/// #[derive(tool, schema)]
/// #[tool(
///     rename = "get_weather",
///     desc = "Get the weather at the specified location"
/// )]
/// struct GetWeather {
///     location: String,
/// }
/// ```
///
/// # Examples
///
/// ## Basic Tool derivation
///
/// ```
/// use renfield::tools::{Tool, tool, schema};
///
/// // Must derive both `tool` and `schema`.
///
/// /// Get the current temperature for a city
/// #[derive(tool, schema)]
/// #[tool(rename = "get_temperature")]
/// struct GetTemperature {
///
///     /// The name of the city.
///     city: String,
/// }
///
/// assert_eq!(GetTemperature::as_ollama_tool(), serde_json::json!({
///     "type": "function",
///     "function": {
///         "name": "get_temperature",
///         "description": "Get the current temperature for a city",
///         "parameters": {
///             "type": "object",
///             "description": "Get the current temperature for a city",
///             "required": ["city"],
///             "properties": {
///                 "city": {
///                     "type": "string",
///                     "description": "The name of the city.",
///                 },
///             },
///         },
///     },
/// }));
/// ```
///
pub use codegen::tool;

/// Generate an LLM tool schema for an arbitrary Rust type.
pub trait Tool {

    /// Generate an Anthropic tool schema.
    fn as_anthropic_tool() -> serde_json::Value;

    /// Generate an OpenAI tool schema.
    fn as_openai_tool() -> serde_json::Value;

    /// Generate a Gemini tool schema.
    fn as_gemini_tool() -> serde_json::Value;

    /// Generate an OpenRouter tool schema.
    fn as_openrouter_tool() -> serde_json::Value;

    /// Generate an Ollama tool schema.
    fn as_ollama_tool() -> serde_json::Value;
}
