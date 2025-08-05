use super::response::{Response, Usage, StopReason};
use super::error::AnthropicError;
use super::message::*;

use serde::{Serialize, Deserialize};

/// An Server-Sent-Event message response from the
/// [completion](crate::core::Client::stream_completion) route.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamResponse {

    /// The type of event.
    #[serde(rename = "type")]
    pub kind: StreamEventType,

    /// The index of the content block, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,

    /// Possible error body for [StreamEventType::Error] responses.
    pub error: Option<AnthropicError>,

    /// Possible delta body for [StreamEventType::ContentBlockDelta]
    /// responses.
    pub delta: Option<Delta>,

    /// Content block body.
    pub content_block: Option<Content>,

    /// Possible usage body, which is (usually) included in
    /// `message_delta` events.
    pub usage: Option<Usage>,

    /// Message body for message events;
    pub message: Option<Response>,
}

/// Enumerates the possible types of streaming response events.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamEventType {
    Error,
    MessageStart,
    ContentBlockStart,
    ContentBlockDelta,
    ContentBlockStop,
    MessageDelta,
    MessageStop,
    Ping,
    Other(String),
}

/// Enumerates the possible body types of a [Delta].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeltaKind {
    TextDelta,
    InputJsonDelta,
    ThinkingDelta,
    SignatureDelta,
    ToolUse,
    Text,
    Thinking,
    ServerToolUse,
    WebSearchToolResult,
}

/// Body of a delta streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delta {

    /// The type of content block. None if the event is a
    /// `message_delta`, in which case `stop_reason` is populated.
    #[serde(rename = "type")]
    pub kind: Option<DeltaKind>,

    /// Text body for [DeltaKind::TextDelta] response types.
    pub text: Option<String>,

    /// Text body for [DeltaKind::InputJsonDelta] response types.
    /// This is used for `tool_use` content blocks.
    ///
    /// Current models only support emitting one complete key and
    /// value property from input at a time. As such, when using
    /// tools, there may be delays between streaming events while the
    /// model is working. Once an input key and value are accumulated,
    /// we emit them as multiple `content_block_delta` events with
    /// chunked partial json so that the format can automatically
    /// support finer granularity in future models.
    pub partial_json: Option<String>,

    /// Text body for [DeltaKind::ThinkingDelta] response types.
    pub thinking: Option<String>,

    /// Text body for [DeltaKind::SignatureDelta] response types.
    pub signature: Option<String>,

    /// The reason the content block stream ended.
    pub stop_reason: Option<StopReason>,

    /// Name of the tool being called.
    pub name: Option<String>,
}
