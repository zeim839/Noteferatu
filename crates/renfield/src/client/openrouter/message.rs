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

impl From<VideoInput> for Content {
    fn from(value: VideoInput) -> Self {
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

    /// Video content type.
    InputVideo {

        /// Video input data.
        video_url: VideoInput,
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

impl From<VideoInput> for ContentPart {
    fn from(value: VideoInput) -> Self {
        Self::InputVideo { video_url: value }
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

/// Video input [ContentPart].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoInput {

    /// Video url.
    pub url: String,
}

impl VideoInput {

    /// Create a new [VideoInput] instance from a url.
    pub fn new(url: &str) -> Self {
        Self { url: url.to_string() }
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

#[cfg(test)]
mod tests {
    use crate as renfield;
    use serde_json::{to_value, json};
    use crate::client::openrouter::*;

    #[test]
    fn test_msg_text() {
        let msg = msg!("user", "plain text");
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "user",
            "content": "plain text",
        }));

        let msg = msg!("user", "msg0", "msg1");
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "msg0",
                },
                {
                    "type": "text",
                    "text": "msg1",
                },
            ],
        }));

        let string = String::from("msggg");
        let msg = msg!("system", "static pointer", string);
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "system",
            "content": [
                {
                    "type": "text",
                    "text": "static pointer",
                },
                {
                    "type": "text",
                    "text": "msggg",
                },
            ],
        }));
    }

    #[test]
    fn test_msg_tool_id() {
        let msg = msg!("tool", "my-tool-id", "foo");
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "tool",
            "tool_call_id": "my-tool-id",
            "content": "foo",
        }));
    }

    #[test]
    fn test_msg_image() {
        let msg = msg!("assistant", "what's in this image?", ImageInput::from_url("base64data"));
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "assistant",
            "content": [
                {
                    "type": "text",
                    "text": "what's in this image?",
                },
                {
                    "type": "image_url",
                    "image_url": {
                        "url": "base64data",
                    },
                },
            ],
        }));
    }

    #[test]
    fn test_msg_audio() {
        let audio = AudioInput::new("base64data", AudioFormat::Wav);
        let msg = msg!("developer", "summarize this audio", audio);
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "developer",
            "content": [
                {
                    "type": "text",
                    "text": "summarize this audio",
                },
                {
                    "type": "input_audio",
                    "input_audio": {
                        "data": "base64data",
                        "format": "wav",
                    }
                },
            ],
        }));
    }

    #[test]
    fn test_msg_video() {
        let video = VideoInput::new("video.url");
        let msg = msg!("assistant", "summarize this video", video);
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "assistant",
            "content": [
                {
                    "type": "text",
                    "text": "summarize this video",
                },
                {
                    "type": "input_video",
                    "video_url": {
                        "url": "video.url",
                    },
                }
            ],
        }));
    }
}
