//! Generate tool schemas from custom types.
//!
//! Function calling (also known as tool calling) provides a powerful
//! and flexible way for LLM models to interface with external systems
//! and access data outside their training data.
//!
//! The [`tools`](Self) module lets you define tool schemas from Rust
//! structs.
//!
//! # Examples
//!
//! ## With derive macros
//!
//! ```
//! use renfield::tools::{Tool, tool, tool_info};
//!
//! // A custom tool that retrieves the weather at a specific location.
//! #[derive(tool)]
//! #[tool_info(name = "get_weather", desc = "get the weather at a specific location")]
//! pub struct GetWeather {
//!
//!     #[tool_info(desc = "The city location")]
//!     pub city: String,
//!
//!     // Field descriptions can also be set using docstring.
//!     // (same as using tool_info).
//!
//!     /// The country location.
//!     pub country: String,
//! }
//!
//! fn main() {
//!     let input_schema = GetWeather::parameters_schema();
//!     let schema_str = serde_json::to_string_pretty(&input_schema).unwrap();
//!     println!("input schema: {}", schema_str);
//! }
//! ```
//!
//! ## Define tools manually
//!
//! ```
//! use renfield::tools::Tool;
//! use serde_json::json;
//!
//! // A custom tool that retrieves the weather at a specific location.
//! pub struct GetWeather {
//!     pub city: String,
//!     pub country: String,
//! }
//!
//! impl Tool for GetWeather {
//!     fn name() -> String {
//!         "get_weather".to_string()
//!     }
//!
//!     fn description() -> Option<String> {
//!         Some("get the current weather".to_string())
//!     }
//!
//!     fn parameters_schema() -> serde_json::Value {
//!         json!({
//!             "type": "object",
//!             "properties": {
//!                 "city": {
//!                     "type": "string"
//!                 },
//!                 "country": {
//!                     "type": "string",
//!                 }
//!             },
//!         })
//!     }
//! }
//! ```

/// Derive a [Tool] schema from a struct.
///
/// Generates a [`Tool`] implementation for a custom struct.
///
/// # Examples
///
/// ## Automatic Attributes
///
/// Tools are defined by their [name](Tool::name),
/// [description](Tool::description), and input
/// [parameters](Tool::parameters_schema). By default, the [`tool`]
/// derive macro sets the tool name to the struct name and the
/// description to the struct docstring. Field names, types, and
/// descriptions are likewise derived from the field name & type
/// declaration, and their respective docstrings.
///
/// To manually set tool attributes, see the [next
/// example](#manually-setting-attributes).
///
/// ```
/// use renfield::tools::{Tool, tool};
///
/// /// Docstring becomes the tool's description!
/// #[derive(tool)]
/// pub struct GetStockPrice {
///
///     /// Stock ticker.
///     pub ticker: String,
/// }
///
/// fn main() {
///
///     // Check the tool's name.
///     assert_eq!(<GetStockPrice as Tool>::name(), "GetStockPrice");
///
///     // Check the tool's description.
///     assert_eq!(
///         <GetStockPrice as Tool>::description(),
///         "Docstring becomes the tool's description!"
///     );
///
///     // Check the tool's parameters.
///     let schema = <GetStockPrice as Tool>::parameters_schema();
///     assert_eq!(schema, serde_json::json!({
///         "type": "object",
///         "properties": {
///             "ticker": {
///                 "type": "string",
///                 "description": "Stock ticker.",
///             },
///         },
///         "required": vec!["ticker"],
///     }));
/// }
/// ```
///
/// ## Manually Setting Attributes
///
/// You can use the [`tool_info`] attribute macro in conjunction with
/// [`tool`] to manually set tool attributes.
///
/// ```
/// use renfield::tools::{tool, tool_info};
///
/// /// This docstring no longer serves as a tool description.
/// #[derive(tool)]
/// #[tool_info(name = "get_stock_price", desc = "tool description")]
/// pub struct GetStockPrice {
///
///     /// This docstring is not a field description!
///     #[tool_info(desc = "this is the field description")]
///     pub ticker: String,
/// }
/// ```
pub use codegen::tool;

/// Customize [Tool] metadata.
///
/// Explicitly specify a tool's name and description, as well as the
/// names and descriptions of its respective fields.
///
/// **MUST BE** used in conjunction with [`tool`].
///
/// # Example
///
/// ```
/// use renfield::tools::{tool, tool_info};
///
/// /// Docstring descriptions are overwritten!
/// #[derive(tool)]
/// #[tool_info(name = "my_tool", desc = "set the tool's description")]
/// pub struct MyTool {
///
///     #[tool_info(name = "renamed_field", desc = "field description")]
///     pub old_field_name: u64,
/// }
/// ```
pub use codegen::tool_info;

/// Tool definition.
///
/// A tool definition consists of a tool name, an optional
/// description, and a [JSON
/// schema](https://json-schema.org/understanding-json-schema/reference)
/// of its input arguments.
///
/// The tool object spec varies across [LLM
/// clients](crate::client). For example, OpenAI uses the `parameters`
/// field for input arguments, whereas Anthropic uses
/// `input_schema`. Explicitly defining a tool's `name`,
/// `description`, and `parameters_schema` ensures that a valid
/// schema can be generated for all LLM clients.
pub trait Tool {

    /// Name.
    ///
    /// Tool names must be unique to distinguish between
    /// implementations.
    fn name() -> String;

    /// Description.
    ///
    /// Instructs the LLM when and how to call the tool.
    fn description() -> Option<String>;

    /// Generate a JSON schema for the tool's input arguments.
    fn parameters_schema() -> serde_json::Value;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_derive_without_attributes() {

        /// Tool description.
        #[derive(tool)]
        struct GetWeather {

            /// Location city.
            pub city: String,

            /// Location country.
            pub country: String,
        }

        assert_eq!(<GetWeather as Tool>::name(), "GetWeather");
        assert!(<GetWeather as Tool>::description().is_some_and(
            |desc| desc == "Tool description."
        ));

        let schema = <GetWeather as Tool>::parameters_schema();
        assert!(schema.get("type").is_some_and(|kind| kind == "object"));
        assert!(schema.get("properties").is_some_and(
            |props| matches!(props, serde_json::Value::Object(_))
        ));

        let props = schema["properties"];
        assert!(props.get("city").is_some_and(|city_obj| {
            matches!(city_obj, serde_json::Value::Object(_)) &&
            city_obj.get("type").is_some_and(|kind| kind == "string") &&
            city_obj.get("description").is_some_and(
                |desc| desc == "Location city."
            )
        }));

        assert!(props.get("country").is_some_and(|country_obj| {
            matches!(country_obj, serde_json::Value::Object(_)) &&
            country_obj.get("type").is_some_and(|kind| kind == "string") &&
            country_obj.get("description").is_some_and(
                |desc| desc == "Location country."
            )
        }));
    }

    #[test]
    fn test_custom_tool_name() {
        todo!();
    }

    #[test]
    fn test_custom_tool_description() {
        todo!();
    }

    #[test]
    fn test_custom_field_name() {
        todo!();
    }

    #[test]
    fn test_custom_field_description() {
        todo!();
    }

    #[test]
    fn test_tool_info_without_derive() {
        // tool_info should do nothing without derive(tool).
        todo!();
    }

    #[tokio::test]
    async fn test_with_client() {
        todo!();
    }

    #[tokio::test]
    async fn test_with_agent() {
        todo!();
    }
}
