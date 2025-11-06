//! Message contents.

use serde::{Serialize, Deserialize};
use super::tools::ToolCall;

/// Conveniently construct an OpenRouter [Message].
pub use macros::openrouter_msg as msg;

/// Conversation message.
///
/// See: [API Reference](https://openrouter.ai/docs/api-reference/chat/send-chat-completion-request#request.body.messages)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum Message {

    /// Developer-provided instructions that the model should follow,
    /// regardless of messages sent by the user.
    System {

        /// The contents of the system message.
        content: Content,

        /// An optional participant name.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },

    /// Messages sent by an end user, containing prompts or additional
    /// context information.
    User {

        /// The contents of the user message.
        content: Content,

        /// An optional participant name.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },

    /// Developer-provided instructions that the model should follow,
    /// regardless of messages sent by the user.
    Developer {

        /// The contents of the developer message.
        content: Content,

        /// An optional participant name.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },

    /// Messages sent by the model in response to user messages.
    Assistant {

        /// The contents of the assistant message.
        ///
        /// Required unless `tool_calls` is specified.
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<Content>,

        /// An optional participant name.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,

        /// Tool calls made by the assistant.
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,

        /// Reason the request was refused.
        #[serde(skip_serializing_if = "Option::is_none")]
        refusal: Option<String>,

        /// Internal reasoning trace.
        #[serde(skip_serializing_if = "Option::is_none")]
        reasoning: Option<String>,
    },

    /// Tool call outputs.
    Tool {

        /// The contents of the tool message.
        content: Content,

        /// The tool call this message is responding to.
        tool_call_id: String,
    }
}

/// The contents of a [Message].
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Content {

    /// Plain-text content.
    Text(String),

    /// A list of content parts.
    ContentParts(Vec<ContentPart>),
}

impl Content {

    /// Combine `self` and `other` into a single [Content],
    /// consuming `other`.
    pub fn combine(self, other: Self) -> Self {
        let mut new_parts = Vec::new();
        match self {
            Content::Text(text) => {
                new_parts.push(ContentPart::from(text));
            },
            Content::ContentParts(mut parts) => {
                new_parts.append(&mut parts);
            },
        }
        match other {
            Content::Text(text) => {
                new_parts.push(ContentPart::from(text));
            },
            Content::ContentParts(mut parts) => {
                new_parts.append(&mut parts);
            },
        }
        Self::ContentParts(new_parts)
    }
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<ImageInput> for Content {
    fn from(value: ImageInput) -> Self {
        Self::ContentParts(vec![ContentPart::from(value)])
    }
}

impl From<AudioInput> for Content {
    fn from(value: AudioInput) -> Self {
        Self::ContentParts(vec![ContentPart::from(value)])
    }
}

impl From<Vec<ContentPart>> for Content {
    fn from(value: Vec<ContentPart>) -> Self {
        Self::ContentParts(value)
    }
}

/// One of possibly many [Content] parts.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {

    /// Plain-text content.
    Text {
        text: String,
    },

    /// Image data content.
    ImageUrl {

        /// Image object data.
        image_url: ImageInput,
    },

    /// Audio content part.
    InputAudio {

        /// Audio input data.
        input_audio: AudioInput,
    },
}

impl From<&str> for ContentPart {
    fn from(value: &str) -> Self {
        Self::Text { text: value.to_string() }
    }
}

impl From<String> for ContentPart {
    fn from(value: String) -> Self {
        Self::Text { text: value }
    }
}

impl From<ImageInput> for ContentPart {
    fn from(value: ImageInput) -> Self {
        Self::ImageUrl { image_url: value }
    }
}

impl From<AudioInput> for ContentPart {
    fn from(value: AudioInput) -> Self {
        Self::InputAudio { input_audio: value }
    }
}

/// Image input [ContentPart].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageInput {

    /// Either a URL of the image or the base64 encoded image data.
    pub url: String,

    /// Specifies the detail level of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl ImageInput {

    /// Create an [ImageInput] from a URL of the image or a string of
    /// its base64-encoded data.
    pub fn from_url(data: &str) -> Self {
        ImageInput { url: data.to_string(), detail: None }
    }
}

/// Audio input [ContentPart].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioInput {

    /// Base64 encoded audio data.
    pub data: String,

    /// The format of the encoded audio data.
    pub format: AudioFormat,
}

impl AudioInput {

    /// Create a new [InputAudio] instance.
    ///
    /// `data` is a base64-encoded string of audio data.
    pub fn new(data: &str, format: AudioFormat) -> Self {
        Self { data: data.to_string(), format }
    }
}

/// The format of encoded audio data.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    Wav,
    Mp3,
    Flac,
    M4a,
    Ogg,
    Pcm16,
    Pcm24,
}
