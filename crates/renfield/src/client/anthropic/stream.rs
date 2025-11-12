//! Streaming events.

use super::error::Error;
use super::message::ContentPart;
use super::response::{Response, Usage, StopReason};
use serde::Deserialize;

/// A server-sent chat generation event.
///
/// See: [API Reference](https://docs.claude.com/en/docs/build-with-claude/streaming)
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamEvent {
    MessageStart {
        message: Response,
    },
    ContentBlockStart {
        index: u64,
        content_block: ContentPart,
    },
    ContentBlockDelta {
        index: u64,
        delta: Delta,
    },
    ContentBlockStop {
        index: u64,
    },
    MessageDelta {
        delta: MessageDelta,
        usage: Usage,
    },
    MessageStop,
    Error {
        error: Error,
    },
    Ping,
}

/// [StreamEvent] content delta.
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Delta {
    TextDelta { text: String },
    InputJsonDelta { partial_json: String },
    ThinkingDelta { thinking: String },
    SignatureDelta { signature: String },
}

/// Message delta [StreamEvent] contents.
#[derive(Deserialize, Debug, Clone)]
pub struct MessageDelta {
    pub stop_reason: Option<StopReason>,
    pub stop_sequence: Option<String>,
}
