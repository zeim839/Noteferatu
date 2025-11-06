//! Message contents.
//!
//! A [Message] forms part of a conversation with an LLM. The
//! conversation history is a vector of messages, ending with a
//! `user`, `developer`, or `system` message, that is passed to a chat
//! completion [Request](super::Request) to generate a response from
//! the LLM.
//!
//! # Getting Started
//!
//! The [Message] enum consists of variants representing the possible
//! message fields for each role. The role of a message can be one of
//! `Developer`, `System`, `User`, `Assistant`, or `Tool`, and
//! indicates the sender of the message. Some message variants, like
//! `Assistant` or `Tool` contain fields that are otherwise prohibited
//! in other roles.
//!
//! Each message consists of some [Content]. Message content can
//! either be plain text or a vector of [ContentPart]s. [ContentPart]
//! supports text, image, file, and audio message data.
//!
//! # Content Rules
//!
//! The OpenAI completions API supports a number of message roles and
//! content types. Each role only allows certain [ContentPart]
//! variants. Renfield does not automatically validate messages;
//! illegal messages will cause the [Client](super::Client) to fail
//! with an API error.
//!
//! * [`Developer`](Message::Developer) messages only support text.
//! * [`System`](Message::System) messages only support text.
//! * [`User`](Message::User) messages support all content types
//!   (except refusal).
//! * [`Assistant`](Message::Assistant) messages support text &
//!   refusal.
//! * [`Tool`](Message::Tool) messages support only text.
//!
//! # The [`msg`] macro
//!
//! Use the [`msg`] macro to succinctly create OpenAI
//! messages. Specify a role, and then pass any number of String/str
//! literals or [ContentPart]s to compose a message.
//!
//! ```
//! use renfield::client::openai::*;
//!
//! let image = ContentPart::from(ImageInput::from_url("data"));
//! let file = ContentPart::from(FileInput::from_file_data("my-file.txt", "data"));
//!
//! let my_message = msg!("user", "Describe this image and file", image, file);
//! ```
//!
//! # API Reference
//!
//! [OpenAI Create chat completion (messages)](https://platform.openai.com/docs/api-reference/chat/create#chat_create-messages)

use serde::{Serialize, Deserialize};

use super::request::AudioFormat;

/// Conveniently construct an OpenAI [Message].
///
/// The [`msg`] macro takes a role string and a series of content
/// parts and constructs an OpenAI [`Message`].
///
/// # Examples
///
/// ## Basic text messages
///
/// ```
/// use renfield::client::openai::msg;
/// use serde_json::{json, to_value};
///
/// let msg_json = to_value(msg!("user", "Hello!")).unwrap();
/// assert_eq!(msg_json, json!({
///     "role": "user",
///     "content": "Hello!",
/// }));
/// ```
///
/// ## Multimedia Messages
///
/// Multimedia messages are messages containing images, input audio,
/// or files. Optionally, they may also include accompanying
/// text. Please note that the OpenAI client only supports multimedia
/// messages from the "user" role.
///
/// ```
/// use renfield::client::openai::message::*;
/// use serde_json::{json, to_value};
///
/// // Example: file data input.
/// let base64data = String::from("base-64-encoded-data");
/// let file = FileInput::from_file_data("my-file.txt", &base64data);
/// let json_msg = to_value(msg!("user", "what's in this file?", file)).unwrap();
/// assert_eq!(json_msg, json!({
///     "role": "user",
///     "content": [
///         {
///             "type": "text",
///             "text": "what's in this file?",
///         },
///         {
///             "type": "file",
///             "file": {
///                 "file_data": "base-64-encoded-data",
///                 "filename": "my-file.txt",
///             },
///         },
///     ],
/// }));
///
/// // Example: image data input.
/// let base64data = String::from("base-64-encoded-image");
/// let image = ImageInput::from_url(&base64data);
/// let json_msg = to_value(msg!("user", "describe this image", image)).unwrap();
/// assert_eq!(json_msg, json!({
///     "role": "user",
///     "content": [
///         {
///             "type": "text",
///             "text": "describe this image",
///         },
///         {
///             "type": "image_url",
///             "image_url": {
///                 "url": "base-64-encoded-image",
///             },
///         },
///     ],
/// }));
/// ```
///
/// ## Multipart Messages
///
/// A multipart message is a single [Message] instance containing
/// multiple text messages or multimedia from the same role. Creating
/// multipart messages is done by passing a variable number of
/// `String` or `ContentPart` arguments to [`msg`].
///
/// ```
/// use renfield::client::openai::*;
///
/// // If you only want text...
/// let plain_text = msg!(
///     "developer",
///     "first text message",
///     "second text message",
///     "third text message"
///     // ...
/// );
///
/// // Or, mix multiple content part types.
/// let image = ImageInput::from_url("data");
/// let file = FileInput::from_file_data("my-file.txt", "data");
/// let multimedia = msg!(
///     "user",
///     "what's in this image?",
///     image,
///     "what's in this file?",
///     file
///     // ...
/// );
/// ```
///
/// ## Tool Messages
///
/// Tool [`msg`] invocations expect an additional macro argument: the
/// tool call ID (which must be a literal or expression that implements
/// `Into<String>`) that the message is responding to.
///
/// ```
/// use renfield::client::openai::msg;
///
/// msg!("tool", "tool-call-id-123", "{ some json response...}");
/// ```
pub use macros::oai_msg as msg;

/// OpenAI completion message.
///
/// See: [API Reference](https://platform.openai.com/docs/api-reference/chat/create#chat_create-messages)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum Message {

    /// Developer-provided instructions that the model should follow,
    /// regardless of messages sent by the user.
    ///
    /// With o1 models and newer, `developer` messages replace the
    /// previous `system` messages.
    Developer {

        /// The contents of the developer message.
        ///
        /// Developer only supports `text` content.
        content: Content,
    },

    /// Developer-provided instructions that the model should follow,
    /// regardless of messages sent by the user.
    ///
    /// With o1 models and newer, use `developer` messages for this
    /// purpose instead.
    System {

        /// The contents of the system message.
        ///
        /// System only supports `text` content.
        content: Content,
    },

    /// Messages sent by an end user, containing prompts or additional
    /// context information.
    User {

        /// The contents of the user message.
        ///
        /// Supports text, image, audio, and file contents.
        content: Content,
    },

    /// Messages sent by the model in response to user messages.
    Assistant {

        /// Data about a previous audio response from the model.
        #[serde(skip_serializing_if = "Option::is_none")]
        audio: Option<AudioResponse>,

        /// The contents of the assistant message.
        ///
        /// Required unless `tool_calls` or `function_call` is
        /// specified.
        ///
        /// Supports text and refusal type content.
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<Content>,

        /// The refusal message by the assistant.
        #[serde(skip_serializing_if = "Option::is_none")]
        refusal: Option<String>,

        /// The tool calls generated by the model, such as function
        /// calls.
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
    },

    /// Tool call outputs.
    Tool {

        /// The contents of the tool message.
        ///
        /// Only supports text content.
        content: Content,

        /// Tool call that this message is responding to.
        tool_call_id: String,
    },
}

/// [Message] content.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Content {
    Text(String),
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

impl From<AudioInput> for Content {
    fn from(value: AudioInput) -> Self {
        Self::ContentParts(vec![ContentPart::from(value)])
    }
}

impl From<FileInput> for Content {
    fn from(value: FileInput) -> Self {
        Self::ContentParts(vec![ContentPart::from(value)])
    }
}

impl From<ImageInput> for Content {
    fn from(value: ImageInput) -> Self {
        Self::ContentParts(vec![ContentPart::from(value)])
    }
}

impl From<ContentPart> for Content {
    fn from(value: ContentPart) -> Self {
        Self::ContentParts(vec![value])
    }
}

impl From<Vec<ContentPart>> for Content {
    fn from(value: Vec<ContentPart>) -> Self {
        Self::ContentParts(value)
    }
}

/// A [Message] content part.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ContentPart {

    /// Text content part.
    Text {

        /// The text content.
        text: String,
    },

    /// Refusal content part.
    Refusal {

        /// The refusal message generated by the model.
        refusal: String,
    },

    /// Image content part.
    ImageUrl {

        /// Image object data.
        image_url: ImageInput,
    },

    /// Audio content part.
    InputAudio {

        /// Audio input data.
        input_audio: AudioInput,
    },

    /// File content part.
    File {

        /// File input data.
        file: FileInput,
    },
}

impl From<&str> for ContentPart {
    fn from(value: &str) -> Self {
        Self::Text { text: value.to_string() }
    }
}

impl From<String> for ContentPart {
    fn from(text: String) -> Self {
        Self::Text { text }
    }
}

impl From<ImageInput> for ContentPart {
    fn from(image_url: ImageInput) -> Self {
        Self::ImageUrl { image_url }
    }
}

impl From<AudioInput> for ContentPart {
    fn from(input_audio: AudioInput) -> Self {
        Self::InputAudio { input_audio }
    }
}

impl From<FileInput> for ContentPart {
    fn from(file: FileInput) -> Self {
        Self::File { file }
    }
}

/// Image input part.
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

/// Input audio.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioInput {

    /// Base64 encoded audio data.
    pub data: String,

    /// The format of the encoded audio data.
    ///
    /// Currently supports "wav" and "mp3".
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

/// Input file object.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInput {

    /// The base64 encoded file data, used when passing the file to
    /// the model as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,

    /// The ID of an uploaded file to use as input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,

    /// The name of the file, used when passing the file to the model
    /// as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

impl FileInput {

    /// Create an [InputFile] from a base64 encoded string.
    pub fn from_file_data(name: &str, data: &str) -> Self {
        Self { file_data: Some(data.to_string()), file_id: None, filename: Some(name.to_string()) }
    }

    /// Create an [InputFile] by referencing an uploaded file.
    pub fn from_file_id(id: &str) -> Self {
        Self { file_data: None, file_id: Some(id.to_string()), filename: None }
    }
}

/// Data about a previous audio response from the model.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AudioResponse {

    /// Unique identifier for a previous audio response from the
    /// model.
    pub id: String,
}

/// A tool call generated by the model.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolCall {

    /// A call to a function tool created by the model.
    Function {

        /// The ID of the tool call.
        id: String,

        /// The function that the model called.
        function: FunctionCall,
    },

    /// A call to a custom tool created by the model.
    Custom {

        /// The ID of the tool call.
        id: String,

        /// A call to a custom tool created by the model.
        custom: CustomCall,
    },
}

/// The function that the model called.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionCall {

    /// The name of the function to call.
    pub name: String,

    /// The arguments to call the function with, as generated by the
    /// model in JSON format.
    ///
    /// Note that the model does not always generate valid JSON, and
    /// may hallucinate parameters not defined by your function
    /// schema. Validate the arguments in your code before calling
    /// your function.
    pub arguments: String,
}

/// A call to a custom tool created by the model.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomCall {

    /// The input for the custom tool call generated by the model.
    pub input: String,

    /// The name of the custom tool to call.
    pub name: String,
}

#[cfg(test)]
mod tests {
    use crate as renfield;
    use serde_json::{to_value, json};
    use crate::client::openai::*;

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
    fn test_msg_file() {
        let file = FileInput::from_file_data("my-file.txt", "data");
        let msg = msg!("user", "what's in this file?", file);
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "what's in this file?",
                },
                {
                    "type": "file",
                    "file": {
                        "file_data": "data",
                        "filename": "my-file.txt",
                    },
                },
            ],
        }));
    }
}
