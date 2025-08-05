use serde::{Serialize, Deserialize};
use crate::core;

/// The base structured datatype containing multi-part content of a
/// message.
///
/// A `Content` includes a `role` field designating the producer of
/// the `Content` and a `parts` field containing multi-part data that
/// contains the content of the message turn.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    #[serde(default)]
    pub parts: Vec<Part>,
    pub role: Role,
}

/// Specifies the producer of message content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Model,
    Function,
}

impl From<core::Role> for Role {
    fn from(value: core::Role) -> Self {
        match value {
            core::Role::System | core::Role::User => Self::User,
            core::Role::Assistant => Self::Model,
            core::Role::Tool => Self::Function,
        }
    }
}

/// A datatype containing media that is part of a multi-part Content
/// message.
///
/// A Part consists of data which has an associated datatype. A Part
/// can only contain one of the accepted types in Part.data.
///
/// A Part must have a fixed IANA MIME type identifying the type and
/// subtype of the media if the inlineData field is filled with raw
/// bytes.
///
/// API Reference: [Part](https://ai.google.dev/api/caching#Part)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Part {

    /// Indicates if the part is thought from the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought: Option<bool>,

    /// An opaque signature for the thought so it can be reused in
    /// subsequent requests.
    ///
    /// A base64-encoded string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought_signature: Option<String>,

    /// The data contained by this [Part].
    #[serde(flatten)]
    pub data: PartData,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartData {

    /// Inline text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// A predicted FunctionCall with arguments and their values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCallPartData>,

    /// The result output of a `FunctionCall`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response: Option<FunctionResponsePartData>,
}

impl PartData {

    /// Instantiates a new [PartData] with all `None`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Populates the [Self::text] field with the given value.
    pub fn with_text(self, text: Option<String>) -> Self {
        Self { text, ..self}
    }

    /// Populates the [Self::function_call] field with the given value.
    pub fn with_function_call(self, function_call: Option<FunctionCallPartData>) -> Self {
        Self { function_call, ..self }
    }

    /// Populates the [Self::function_response] field with the given value.
    pub fn with_function_response(self, function_response: Option<FunctionResponsePartData>) -> Self {
        Self { function_response, ..self }
    }
}

/// A predicted `FunctionCall` with the arguments and their values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallPartData {

    /// The unique id of the function call, which should be referenced
    /// when returning a tool call result.
    pub id: Option<String>,

    /// The name of the function to call.
    pub name: String,

    /// The function parameters and values in JSON object format.
    pub args: Option<serde_json::Value>,
}

/// The result output from a `FunctionCall` that contains a string
/// representing the `FunctionDeclaration.name` and a structured JSON
/// object containing any output from the function is used as context
/// to the model. This should contain the result of a `FunctionCall` made
/// based on model prediction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionResponsePartData {

    /// The id of the function call this response is for. Populated by
    /// the client to match the corresponding function call `id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the function to call.
    pub name: String,

    /// The function response in JSON object format.
    pub response: serde_json::Value,
}

/// Tool details that the model may use to generate response.
///
/// A `Tool` is a piece of code that enables the system to interact with
/// external systems to perform an action, or set of actions, outside
/// of knowledge and scope of the [Model](super::model::Model).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {

    /// A list of `FunctionDeclarations` available to the model that can
    /// be used for function calling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
}

/// Structured representation of a function declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDeclaration {

    /// The name of the function.
    pub name: String,

    /// A brief description of the function.
    pub description: String,

    /// Describes the parameters to the function as a JSON Schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

impl From<core::ToolDefinition> for FunctionDeclaration {
    fn from(value: core::ToolDefinition) -> Self {
        Self {
            name: value.name,
            description: value.description,
            parameters: value.parameters,
        }
    }
}
