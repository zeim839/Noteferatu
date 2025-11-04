//! Conversation messages.

use serde::{Serialize, Deserialize};

/// Conveniently construct an Ollama [Message].
pub use macros::ollama_msg as msg;

/// Message in a conversation between user and assistant.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Message {

    /// Developer-provided instructions that the model should follow,
    /// regardless of messages sent by the user.
    System {

        /// Message text content.
        content: String,
    },

    /// Messages sent by an end user, containing prompts or additional
    /// context information.
    User {

        /// Message text content.
        content: String,

        /// Optional list of inline images for multimodal models.
        ///
        /// Base64-encoded image content.
        #[serde(skip_serializing_if = "Vec::is_empty")]
        images: Vec<String>,
    },

    /// Messages sent by the model in response to user messages.
    Assistant {

        /// Message text content.
        content: String,

        /// Tool call requests produced by the model.
        #[serde(skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,

        /// Optional deliberate thinking trace when `think` is
        /// enabled.
        #[serde(skip_serializing)]
        thinking: Option<String>,

        /// Optional base64-encoded images in the response.
        #[serde(skip_serializing)]
        images: Option<String>,
    },

    /// Tool call outputs.
    Tool {

        /// Message text content.
        content: String,
    },
}

/// Tool calls requested by the assistant.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {

    /// Name of the function to call.
    pub name: String,

    /// What the function does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// JSON object of arguments to pass to the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
}
