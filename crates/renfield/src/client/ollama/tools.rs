//! Tool and function call definitions.

use serde::{Serialize, Deserialize};

/// Tool call requested by the assistant.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {

    /// The function to call.
    pub function: FunctionCall,
}

/// Function call requested by the assistant.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionCall {

    /// Name of the function to call.
    pub name: String,

    /// What the function does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// JSON object of arguments to pass to the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
}
