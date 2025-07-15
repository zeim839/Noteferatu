use super::errors::ClientError;
use super::messages::{Usage, StopReason, MessageResponse};

use serde::{Serialize, Deserialize};
use tokio_stream::Stream;
use std::result::Result;
use std::pin::Pin;
use bytes::Bytes;
use std::task::{Context, Poll};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StreamEvent {
    MessageStart(MessageStart),
    ContentBlockStart(ContentBlockStart),
    ContentBlockDelta(ContentBlockDelta),
    ContentBlockStop(ContentBlockStop),
    MessageDelta(MessageDelta),
    MessageStop(MessageStop),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageStart {
    #[serde(rename = "type")]
    pub kind: String,
    pub message: MessageResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentBlockStart {
    #[serde(rename = "type")]
    pub kind: String,
    pub index: i64,
    pub content_block: ContentBlock,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub kind: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentBlockDelta {
    #[serde(rename = "type")]
    pub kind: String,
    pub index: i64,
    pub delta: ContentBlock,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentBlockStop {
    #[serde(rename = "type")]
    pub kind: String,
    pub index: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDelta {
    #[serde(rename = "type")]
    pub kind: String,
    pub delta: MessageDeltaInner,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDeltaInner {
    pub stop_reason: StopReason,
    pub stop_sequence: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageStop {
    #[serde(rename = "type")]
    pub kind: String,
}

/// Implements Server-Sent Events (SSE) streaming for Anthropic
/// streaming completions.
pub struct StreamSSE {
    inner: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    buffer: String,
}

impl StreamSSE {

    /// Create a new [StreamSSE] instance.
    pub(crate) fn new(stream: impl Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static) -> Self {
        Self {
            inner: Box::pin(stream),
            buffer: String::new(),
        }
    }

    fn parse_next_event(&mut self) -> Option<Result<StreamEvent, ClientError>> {
        // Look for complete SSE events (separated by double newlines)
        while let Some(double_newline_pos) = self.buffer.find("\n\n") {
            let event_block = self.buffer[..double_newline_pos].to_string();
            self.buffer.drain(..=double_newline_pos + 1);

            // Parse the event block line by line
            for line in event_block.lines() {
                let line = line.trim();

                // Skip empty lines and non-data lines
                if line.is_empty() || !line.starts_with("data: ") {
                    continue;
                }

                // Remove "data: " prefix.
                let data = &line[6..];

                // Try to parse as JSON
                match serde_json::from_str::<StreamEvent>(data) {
                    Ok(response) => return Some(Ok(response)),
                    Err(e) => return Some(Err(ClientError::Json(e))),
                }
            }
        }
        None
    }
}

impl Stream for StreamSSE {
    type Item = Result<StreamEvent, ClientError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            // Try to parse any complete events from the current buffer
            if let Some(result) = self.parse_next_event() {
                return Poll::Ready(Some(result));
            }

            // No complete events available, try to read more data
            match self.inner.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(chunk))) => {
                    match std::str::from_utf8(&chunk) {
                        Ok(chunk_str) => {
                            self.buffer.push_str(chunk_str);
                            // Continue the loop to try parsing again
                        },
                        Err(e) => {
                            return Poll::Ready(Some(Err(ClientError::Stream(e.to_string()))));
                        }
                    }
                },
                Poll::Ready(Some(Err(e))) => {
                    return Poll::Ready(Some(Err(ClientError::Http(e))));
                },
                Poll::Ready(None) => {
                    // Stream ended, try to parse any remaining data
                    if let Some(result) = self.parse_next_event() {
                        return Poll::Ready(Some(result));
                    }

                    // Check if there's any remaining unparsed data
                    if !self.buffer.is_empty() {
                        let remaining = self.buffer.trim();
                        if remaining.starts_with("data: ") {
                            let data = &remaining[6..];
                            if data != "[DONE]" {
                                match serde_json::from_str::<StreamEvent>(data) {
                                    Ok(response) => return Poll::Ready(Some(Ok(response))),
                                    Err(e) => return Poll::Ready(Some(Err(ClientError::Json(e)))),
                                }
                            }
                        }
                    }

                    return Poll::Ready(None);
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}
