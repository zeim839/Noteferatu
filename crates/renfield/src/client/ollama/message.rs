//! Conversation messages.
//!
//! A [Message] forms part of a conversation with an LLM. The
//! conversation history is a vector of messages, ending with a
//! `user`, `system`, `assistant`, or `tool` message that is passed to
//! a [Request](super::Request) to generate a response from the LLM.
//!
//! # The [`msg`] macro
//!
//! The [`msg`] macro uses variadic arguments to reduce the verbosity
//! of instantiating [Message] structs. Compared to other clients,
//! which make better use of their respective [`msg`] implementations,
//! the Ollama variant only accepts text and (when supported) image
//! inputs. If, for this reason, the [`msg`] macro seems redundant,
//! note that it is kept for the sake of consistency with other client
//! APIs (i.e. to avoid introducing unnecessary discrepancies that
//! developers must familiarize themselves with). For the same reason,
//! image data is passed as a tuple, thereby prohibiting developers
//! from making incorrect assumptions of the macro's behavior.
//!
//! The [msg documentation](msg) contains helpful examples.
//!
//! ```
//! use renfield::client::ollama::msg;
//!
//! // Plain text.
//! let text_msg = msg!("user", "hello, world!");
//!
//! // User and assistant roles support image inputs (model support varies).
//! let image_base64 = "abcdef0123...";
//! let user_img = msg!("user", "some prompt", (image_base64, image_base64));
//! let assi_img = msg!("assistant", "some prompt", (image_base64));
//! ```

use serde::{Serialize, Deserialize};
use super::tools::ToolCall;

/// Conveniently construct an Ollama [Message].
///
/// The [`msg`] macro simplifies the instantiation of [Message]
/// structs by accepting message contents as variadic inputs. Unlike
/// other clients, Ollama messages support only text and, in special
/// cases, image inputs. Likewise, Ollama does not support
/// e.g. [ContentPart](renfield::client::openai::ContentPart) like
/// OpenAI or OpenRouter do, meaning that **a message can consist of at
/// most a single text string**.
///
/// Considering these circumstances, it is fair to say that Ollama's
/// [`msg`] implementation is redundant. It is nonetheless kept to
/// avoid overburdening the developer with inconsistencies across
/// client implementations. For the same reason, image inputs are
/// passed as a separate tuple list, thereby preventing developers
/// from making false assumptions of the macro's behavior.
///
/// # Examples
///
/// ## Text Messages
///
/// ```
/// use renfield::client::ollama::msg;
/// use serde_json::{json, to_value};
///
/// // Text messages are supported by all roles (system, user,
/// // assistant, tool).
/// let text_msg = msg!("user", "Hello, world!");
/// assert_eq!(to_value(text_msg).unwrap(), json!({
///     "role": "user",
///     "content": "Hello, world!",
/// }));
/// ```
///
/// ## Image Messages
///
/// Messages containing images are supported by the
/// [User](Message::User) and [Assistant](Message::Assistant) roles.
///
/// ```
/// use renfield::client::ollama::msg;
/// use serde_json::{json, to_value};
///
/// // Image bytes as base64 string.
/// let base64_data = "base64 image data ...";
///
/// // Attach as many images as needed in tuple.
/// let msg = msg!("user", "describe these images", (base64_data, base64_data));
/// assert_eq!(to_value(msg).unwrap(), json!({
///     "role": "user",
///     "content": "describe these images",
///     "images": [
///         "base64 image data ...",
///         "base64 image data ...",
///     ],
/// }));
/// ```
///
pub use macros::ollama_msg as msg;

/// Message in a conversation between user and assistant.
///
/// See: [API Reference](https://docs.ollama.com/api/chat#body-messages)
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
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
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
        images: Option<Vec<String>>,
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
