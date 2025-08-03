use serde::{Serialize, Deserialize};
use super::tools::*;

/// The role of the messages author.
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    #[default]
    Assistant,
    Tool,
}

/// LLM message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: MessageContent,
}

/// LLM message content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {

    /// Text content represents a single text-only message.
    Text(String),

    /// Call to a named function along with its input arguments.
    ToolCall(ToolCall),

    /// Response from calling a tool.
    ToolResponse(ToolResponse),
}
