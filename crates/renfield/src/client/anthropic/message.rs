//! Message contents.

use serde::{Deserialize, Serialize};
use super::tools::*;

/// Conveniently construct an Anthropic [Message].
pub use macros::anthropic_msg as msg;

/// A message that forms part of a conversation with an assistant.
///
/// There is no `system` role type. Instead, use the top-level
/// [`system`](super::Request::system) parameter.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum Message {

    /// User message.
    User { content: Content },

    /// Assistant message.
    Assistant { content: Content },
}

/// [Message] content.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Content {
    Text(String),
    Parts(Vec<ContentPart>),
}

impl Content {

    /// Combines two [Content] values into a single value.
    pub fn combine(self, other: Self) -> Self {
        let mut new_parts = Vec::new();
        match self {
            Content::Text(text) => new_parts.push(ContentPart::from(text)),
            Content::Parts(mut parts) => new_parts.append(&mut parts),
        }
        match other {
            Content::Text(text) => new_parts.push(ContentPart::from(text)),
            Content::Parts(mut parts) => new_parts.append(&mut parts),
        }
        Content::Parts(new_parts)
    }
}

impl From<ContentPart> for Content {
    fn from(value: ContentPart) -> Self {
        Self::Parts(vec![value])
    }
}

impl From<Vec<ContentPart>> for Content {
    fn from(value: Vec<ContentPart>) -> Self {
        Self::Parts(value)
    }
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

/// Part of a series of [Content] values.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text(Text),
    Image(Image),
    Document{ source: Document },
    SearchResult(SearchResult),
    Thinking(Thinking),
    RedactedThinking(RedactedThinking),
    ToolUse(ToolUse),
    ToolResult(ToolResult),
    ServerToolUse(ServerToolUse),
    WebSearchToolResult(WebSearchToolResult),
    WebFetchToolResult(WebFetchToolResult),
    CodeExecutionToolResult(CodeExecutionToolResult),
    BashCodeExecutionToolResult(BashCodeExecutionToolResult),
    TextEditorCodeExecutionToolResult(TextEditorCodeExecutionToolResult),
    McpToolUse(McpToolUse),
    McpToolResult(McpToolResult),
}

impl From<String> for ContentPart {
    fn from(value: String) -> Self {
        Self::Text(Text::from(value))
    }
}

impl From<&str> for ContentPart {
    fn from(value: &str) -> Self {
        Self::Text(Text::from(value))
    }
}

impl From<Text> for ContentPart {
    fn from(value: Text) -> Self {
        Self::Text(value)
    }
}

impl From<Image> for ContentPart {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}

impl From<Document> for ContentPart {
    fn from(value: Document) -> Self {
        Self::Document { source: value }
    }
}

impl From<SearchResult> for ContentPart {
    fn from(value: SearchResult) -> Self {
        Self::SearchResult(value)
    }
}

impl From<Thinking> for ContentPart {
    fn from(value: Thinking) -> Self {
        Self::Thinking(value)
    }
}

impl From<RedactedThinking> for ContentPart {
    fn from(value: RedactedThinking) -> Self {
        Self::RedactedThinking(value)
    }
}

impl From<ToolUse> for ContentPart {
    fn from(value: ToolUse) -> Self {
        Self::ToolUse(value)
    }
}

impl From<ToolResult> for ContentPart {
    fn from(value: ToolResult) -> Self {
        Self::ToolResult(value)
    }
}

impl From<ServerToolUse> for ContentPart {
    fn from(value: ServerToolUse) -> Self {
        Self::ServerToolUse(value)
    }
}

impl From<WebSearchToolResult> for ContentPart {
    fn from(value: WebSearchToolResult) -> Self {
        Self::WebSearchToolResult(value)
    }
}

impl From<WebFetchToolResult> for ContentPart {
    fn from(value: WebFetchToolResult) -> Self {
        Self::WebFetchToolResult(value)
    }
}

impl From<CodeExecutionToolResult> for ContentPart {
    fn from(value: CodeExecutionToolResult) -> Self {
        Self::CodeExecutionToolResult(value)
    }
}

impl From<BashCodeExecutionToolResult> for ContentPart {
    fn from(value: BashCodeExecutionToolResult) -> Self {
        Self::BashCodeExecutionToolResult(value)
    }
}

impl From<TextEditorCodeExecutionToolResult> for ContentPart {
    fn from(value: TextEditorCodeExecutionToolResult) -> Self {
        Self::TextEditorCodeExecutionToolResult(value)
    }
}

impl From<McpToolUse> for ContentPart {
    fn from(value: McpToolUse) -> Self {
        Self::McpToolUse(value)
    }
}

impl From<McpToolResult> for ContentPart {
    fn from(value: McpToolResult) -> Self {
        Self::McpToolResult(value)
    }
}

/// Text [ContentPart].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Text {

    /// The text content.
    pub text: String,

    /// Create a cache control breakpoint at this content block.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,

    /// Citations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub citations: Vec<Citation>,
}

impl Text {

    /// Create a new [Text] instance.
    pub fn new(text: &str, cache_control: Option<CacheControl>, citations: Option<Vec<Citation>>) -> Self {
        let citations = citations.unwrap_or_default();
        Self { text: text.to_string(), cache_control, citations }
    }
}

impl From<String> for Text {
    fn from(text: String) -> Self {
        Self { text, cache_control: None, citations: Vec::new() }
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

/// Cache control breakpoint.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheControl {

    /// Allowed value: `ephemeral`.
    #[serde(rename = "type")]
    pub kind: String,

    /// The time-to-live for the cache control breakpoint.
    ///
    /// This may be one the following values:
    ///  * `5m`: 5 minutes.
    ///  * `1h`: 1 hour.
    ///
    /// Defaults to `5m`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

/// Content citation.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Citation {
    CharLocation {
        cited_text: String,
        document_index: u64,
        document_title: Option<String>,
        end_char_index: u64,
        start_char_index: u64,
    },
    PageLocation {
        cited_text: String,
        document_index: u64,
        document_title: Option<String>,
        end_page_number: u64,
        start_page_number: u64,
    },
    ContentBlockLocation {
        cited_text: String,
        document_index: u64,
        document_title: Option<String>,
        end_block_index: u64,
        start_block_index: u64,
    },
    WebSearchResultLocation {
        cited_text: String,
        encrypted_index: String,
        title: Option<String>,
        url: String,
    },
    SearchResultLocation {
        cited_text: String,
        end_block_index: u64,
        search_result_index: u64,
        source: String,
        start_block_index: u64,
        title: Option<String>,
    },
}

/// Image [ContentPart].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {

    /// The source data for the image.
    pub source: ImageSource,

    /// Create a cache breakpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
}

impl Image {

    /// Create an [Image] from base64 encoded bytes.
    pub fn base64(data: &str, media_type: MediaType) -> Self {
        Self {
            source: ImageSource::Base64{data: data.to_string(), media_type},
            cache_control: None
        }
    }

    /// Create an [Image] from an image URL.
    pub fn url(url: &str) -> Self {
        Self {
            source: ImageSource::Url{ url: url.to_string() },
            cache_control: None,
        }
    }

    /// Create an [Image] from an uploaded file.
    pub fn file(file_id: &str) -> Self {
        Self {
            source: ImageSource::File{ file_id: file_id.to_string() },
            cache_control: None,
        }
    }
}

/// Source data for an [Image].
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ImageSource {
    Base64 { data: String, media_type: MediaType },
    Url { url: String },
    File { file_id: String },
}

/// [Image] media encoding format.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MediaType {
    #[serde(rename = "image/jpeg")]
    Jpeg,
    #[serde(rename = "image/png")]
    Png,
    #[serde(rename = "image/gif")]
    Gif,
    #[serde(rename = "image/webp")]
    Webp,
    #[serde(rename = "application/pdf")]
    Pdf,
    #[serde(rename = "text/plain")]
    Text,
}

/// Document [ContentPart].
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Document {
    Base64 {
        data: String,
        media_type: MediaType,
    },
    Text {
        data: String,
        media_type: MediaType,
    },
    /// If [Content] is [Part](Content::Part), then only
    /// [Text](ContentPart::Text) and [Image](ContentPart::Image) are
    /// allowed.
    Content(Content),
    #[serde(rename = "url")]
    PdfUrl { url: String },
    File { file_id: String },
}

impl Document {

    /// Create a [Document] from base64 encoded bytes.
    pub fn base64(data: &str, media_type: MediaType) -> Self {
        Self::Base64{data: data.to_string(), media_type}
    }

    /// Create a [Document] from a string.
    pub fn text(data: &str, media_type: MediaType) -> Self {
        Self::Text{data: data.to_string(), media_type}
    }

    /// Create a [Document] from [Content].
    ///
    /// Note: If the [Content] is variant [Part](Content::Part), then
    /// only [Text](ContentPart::Text) and [Image](ContentPart::Image)
    /// are supported.
    pub fn content(content: Content) -> Self {
        Self::Content(content)
    }

    /// Create a [Document] from a PDF URL.
    pub fn pdf_url(url: &str) -> Self {
        Self::PdfUrl{ url: url.to_string() }
    }

    /// Create a [Document] from an uploaded file.
    pub fn file(file_id: &str) -> Self {
        Self::File{file_id: file_id.to_string()}
    }
}

/// Search result [ContentPart].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub content: Text,
    pub source: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
}

impl SearchResult {

    /// Create a new [SearchResult].
    pub fn new<T: Into<Text>>(title: &str, source: &str, content: T) -> Self {
        Self {
            title: title.to_string(),
            source: source.to_string(),
            content: content.into(),
            cache_control: None,
        }
    }
}

/// Thinking [ContentPart].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thinking {
    pub signature: Option<String>,
    pub thinking: String,
}

/// Redacted thinking [ContentPart].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedactedThinking {
    pub data: String,
}

#[cfg(test)]
mod tests {
    use crate as renfield;
    use serde_json::{to_value, json};
    use crate::client::anthropic::*;

    #[test]
    fn test_msg_plain_text() {
        let msg = msg!("user", "plain text");
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "user",
            "content": [{
                "type": "text",
                "text": "plain text",
            }],
        }));
        let msg = msg!("assistant", "plain text");
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "assistant",
            "content": [{
                "type": "text",
                "text": "plain text",
            }],
        }));
    }

    #[test]
    fn test_msg_multipart_text() {
        let msg = msg!("user", "message 0", "message 1");
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "user",
            "content": [{
                "type": "text",
                "text": "message 0",
            }, {
                "type": "text",
                "text": "message 1",
            }],
        }));
        let msg = msg!("assistant", "message 0", "message 1");
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "assistant",
            "content": [{
                "type": "text",
                "text": "message 0",
            }, {
                "type": "text",
                "text": "message 1",
            }],
        }));
    }

    #[test]
    fn test_msg_image() {
        let msg = msg!("user", Image::base64("some data", MediaType::Jpeg));
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "user",
            "content": [{
                "type": "image",
                "source": {
                    "type": "base64",
                    "data": "some data",
                    "media_type": "image/jpeg",
                },
            }]
        }));
        let msg = msg!("assistant", Image::base64("some data", MediaType::Jpeg));
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "assistant",
            "content": [{
                "type": "image",
                "source": {
                    "type": "base64",
                    "data": "some data",
                    "media_type": "image/jpeg",
                },
            }]
        }));
    }

    #[test]
    fn test_msg_document() {
        let msg = msg!("user", Document::base64("some data", MediaType::Pdf));
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "user",
            "content": [{
                "type": "document",
                "source": {
                    "type": "base64",
                    "data": "some data",
                    "media_type": "application/pdf",
                }
            }],
        }));
        let msg = msg!("assistant", Document::base64("some data", MediaType::Pdf));
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "assistant",
            "content": [{
                "type": "document",
                "source": {
                    "type": "base64",
                    "data": "some data",
                    "media_type": "application/pdf",
                }
            }],
        }));
    }

    #[test]
    fn test_msg_tool_use() {
        let tool_use = ToolUse{
            id: "foo".to_string(),
            input: json!(42),
            name: "bar".to_string(),
            cache_control: None,
        };
        let msg = msg!("user", tool_use.clone());
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "user",
            "content": [{
                "type": "tool_use",
                "id": "foo",
                "input": 42,
                "name": "bar",
            }],
        }));
        let msg = msg!("assistant", tool_use);
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "assistant",
            "content": [{
                "type": "tool_use",
                "id": "foo",
                "input": 42,
                "name": "bar",
            }],
        }));
    }

    #[test]
    fn test_msg_tool_result() {
        let tool_result = ToolResult {
            tool_use_id: "some-id".to_string(),
            cache_control: None,
            content: Some(Content::from("hello")),
            is_error: Some(false),
        };
        let msg = msg!("user", tool_result);
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "user",
            "content": [{
                "type": "tool_result",
                "tool_use_id": "some-id",
                "content": "hello",
                "is_error": false,
            }],
        }));
        let tool_result = ToolResult {
            tool_use_id: "some-id".to_string(),
            cache_control: None,
            content: Some(Content::from("hello")),
            is_error: Some(false),
        };
        let msg = msg!("assistant", tool_result);
        assert_eq!(to_value(msg).unwrap(), json!({
            "role": "assistant",
            "content": [{
                "type": "tool_result",
                "tool_use_id": "some-id",
                "content": "hello",
                "is_error": false,
            }],
        }));
    }
}
