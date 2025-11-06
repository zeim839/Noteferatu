//! Conversation messages.

use serde::{Serialize, Deserialize};
use super::tools::ToolCall;

/// Conveniently construct an Ollama [Message].
pub use macros::ollama_msg as msg;

/// Message in a conversation between user and assistant.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "snake_case")]
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
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,

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

#[cfg(test)]
mod tests {
    use crate as renfield;
    use crate::client::ollama::*;
    use serde_json::{to_value, json};

    #[test]
    fn test_msg_system() {
        let msg = msg!("system", "Hello, World!");
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "system",
            "content": "Hello, World!",
        }));
    }

    #[test]
    fn test_msg_user_single() {
        let msg = msg!("user", "Hello, World!");
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "user",
            "content": "Hello, World!",
        }));
    }

    #[test]
    fn test_msg_user_multiple() {
        let msg = msg!("user", "Hello, World!", ("first-img", "second-img"));
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "user",
            "content": "Hello, World!",
            "images": ["first-img", "second-img"],
        }));
    }

    #[test]
    fn test_msg_assistant() {
        let msg = msg!("assistant", "Hello, World!");
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "assistant",
            "content": "Hello, World!",
        }));
    }

    #[test]
    fn test_msg_tool() {
        let msg = msg!("tool", "Hello, World!");
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "tool",
            "content": "Hello, World!",
        }));
    }
}
