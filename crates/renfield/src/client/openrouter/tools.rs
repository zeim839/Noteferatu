//! Tool calls & tool calling configuration.

use serde::{Serialize, Deserialize};

/// A call to a tool made by the LLM assistant.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {

    /// The tool call ID.
    pub id: String,

    /// The function called.
    #[serde(flatten)]
    pub function: FunctionCall,
}

/// A function called by a [ToolCall].
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "function", rename_all = "snake_case")]
pub enum FunctionCall {
    Function {

        /// The name of the function that was called.
        name: String,

        /// Arguments to the function.
        arguments: String,
    }
}

/// Configure how the model should call tools.
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ToolChoiceConfig {
    Mode(ToolChoiceMode),
    Object(ToolCallChoice)
}

impl From<ToolChoiceMode> for ToolChoiceConfig {
    fn from(value: ToolChoiceMode) -> Self {
        Self::Mode(value)
    }
}

impl From<ToolCallChoice> for ToolChoiceConfig {
    fn from(value: ToolCallChoice) -> Self {
        Self::Object(value)
    }
}

/// Tool calling mode.
#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceMode {

    /// The model is prohibited from calling tools.
    None,

    /// The model decides when to call tools.
    Auto,

    /// The model must call a tool.
    Required,
}

/// Specifies a function that the model must call.
///
/// Part of [ToolChoiceConfig].
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolCallChoice {

    /// A function the model is required to call.
    Function {
        name: String,
    }
}
