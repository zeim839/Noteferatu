use serde::{Serialize, Deserialize};
use crate::core::{self, Role, MessageContent};

/// A message to Claude.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Vec<Content>,
}

/// Message content type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ContentKind {
    #[default]
    Text,
    Thinking,
    RedactedThinking,
    ToolUse,
    ServerToolUse,
    WebSearchToolUse,
    CodeExecutionToolResult,
    McpToolUse,
    McpToolResult,
    ContainerUpload,
    ToolResult,
}

/// Enumerates the possible content types of a
/// [Response](super::Response).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Content {

    /// The type of content response.
    #[serde(rename = "type")]
    pub kind: ContentKind,

    /// Text response data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Input to a tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,

    /// Name of a tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Tool use instance identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_use_id: Option<String>,

    /// The content of a `tool_result` block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the tool call resulted in an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

impl From<core::Message> for Message {
    fn from(value: core::Message) -> Self {
        let content = match value.content {
            MessageContent::Text(text) => vec![Content {
                kind: ContentKind::Text,
                text: Some(text),
                ..Default::default()
            }],
            MessageContent::ToolCall(tool_call) => vec![Content {
                kind: ContentKind::ToolUse,
                tool_use_id: Some(tool_call.id),
                name: Some(tool_call.name),
                input: Some(tool_call.arguments),
                ..Default::default()
            }],
            MessageContent::ToolResponse(tool_response) => vec![Content {
                kind: ContentKind::ToolResult,
                tool_use_id: Some(tool_response.id),
                content: Some(tool_response.content),
                is_error: Some(false),
                ..Default::default()
            }],
        };

        Self {
            role: value.role,
            content,
        }
    }
}
