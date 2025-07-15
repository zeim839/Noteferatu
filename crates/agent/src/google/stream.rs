use super::errors::ClientError;
use super::chat::ChatResponse;

use tokio_stream::Stream;
use std::result::Result;
use std::pin::Pin;
use bytes::Bytes;
use std::task::{Context, Poll};

/// Implements streaming for Google Gemini API completions.
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

    /// Parses the next complete JSON object from the buffer.
    ///
    /// Google's streaming API returns a JSON array of ChatResponse objects.
    /// Each chunk might contain partial objects, or multiple objects, along
    /// with array delimiters `[` and `]` and commas `,`.
    fn parse_next_event(&mut self) -> Option<Result<ChatResponse, ClientError>> {
        println!("parse_next_event: Current buffer state: \"{}\"", self.buffer);

        let mut start_idx = 0;
        let mut brace_count = 0;
        let mut object_start = None;

        // Skip leading whitespace and array opening bracket
        while start_idx < self.buffer.len() && (self.buffer.as_bytes()[start_idx] as char).is_ascii_whitespace() {
            start_idx += 1;
        }
        if start_idx < self.buffer.len() && self.buffer.as_bytes()[start_idx] == b'[' {
            start_idx += 1;
        }

        // Find the start of a JSON object '{'
        for i in start_idx..self.buffer.len() {
            match self.buffer.as_bytes()[i] {
                b'{' => {
                    if object_start.is_none() {
                        object_start = Some(i);
                    }
                    brace_count += 1;
                },
                b'}' => {
                    brace_count -= 1;
                    if brace_count == 0 && object_start.is_some() {
                        // Found a complete JSON object
                        let obj_start = object_start.unwrap();
                        let obj_end = i + 1; // Inclusive '}'

                        let json_str_to_parse = &self.buffer[obj_start..obj_end];

                        println!("Attempting to parse: \"{}\"", json_str_to_parse);
                        match serde_json::from_str::<ChatResponse>(json_str_to_parse) {
                            Ok(response) => {
                                // After parsing, remove the parsed object and any trailing comma or array closing bracket
                                let mut drain_end = obj_end;
                                // Check for comma or ']' after the object
                                while drain_end < self.buffer.len() && (self.buffer.as_bytes()[drain_end] as char).is_ascii_whitespace() {
                                    drain_end += 1;
                                }
                                if drain_end < self.buffer.len() && self.buffer.as_bytes()[drain_end] == b',' {
                                    drain_end += 1;
                                } else if drain_end < self.buffer.len() && self.buffer.as_bytes()[drain_end] == b']' {
                                    // This ']' might be the final one, or intermediate if multiple are closing
                                    // Let's not consume it prematurely unless it's truly the end of the stream.
                                    // For now, if we found a valid object, just consume up to the object + any following comma
                                    // The end of stream will be handled in poll_next when `self.inner` returns None.
                                }

                                self.buffer.drain(..drain_end);
                                println!("Parsed successfully. Remaining buffer: \"{}\"", self.buffer);
                                return Some(Ok(response));
                            },
                            Err(e) => {
                                // If parsing a seemingly complete object fails, it's an error.
                                // We should report it and clear the buffer to prevent infinite loops on bad data.
                                println!("JSON parsing error: {:?} for \"{}\"", e, json_str_to_parse);
                                self.buffer.clear(); // Clear the bad data
                                return Some(Err(ClientError::Json(e)));
                            }
                        }
                    }
                },
                // Handle potential array start/end or other non-JSON object chars
                b'[' | b']' | b',' => {
                    // Ignore these outside of brace counting, as they are array delimiters/separators.
                    // If we are at the very beginning and see '[', we should advance start_idx.
                }
                _ => {} // Other characters are part of the JSON content
            }
        }
        None
    }
}

impl Stream for StreamSSE {
    type Item = Result<ChatResponse, ClientError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            // Try to parse any complete events from the current buffer
            if let Some(result) = self.parse_next_event() {
                return Poll::Ready(Some(result));
            }

            // No complete events available, try to read more data
            println!("poll_next: No complete events, polling inner stream.");
            match self.inner.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(chunk))) => {
                    match std::str::from_utf8(&chunk) {
                        Ok(chunk_str) => {
                            println!("poll_next: Received chunk: \"{}\"", chunk_str);
                            self.buffer.push_str(chunk_str);
                            // Continue the loop to try parsing again
                        },
                        Err(e) => {
                            println!("poll_next: UTF-8 conversion error: {}", e);
                            return Poll::Ready(Some(Err(ClientError::Stream(e.to_string()))));
                        }
                    }
                },
                Poll::Ready(Some(Err(e))) => {
                    println!("poll_next: HTTP error on chunk: {:?}", e);
                    return Poll::Ready(Some(Err(ClientError::Http(e))));
                },
                Poll::Ready(None) => {
                    // Inner stream ended. Try one last time to parse any remaining data.
                    // This is crucial for the very last JSON object which might not be followed by a comma.
                    println!("poll_next: Inner stream ended. Final buffer: \"{}\"", self.buffer);
                    if let Some(result) = self.parse_next_event() {
                        return Poll::Ready(Some(result));
                    }

                    // If there's still unparseable data (e.g., just `]` or incomplete JSON)
                    // after the stream ends and no more objects can be parsed, clear it and end.
                    if !self.buffer.trim().is_empty() && self.buffer.trim() != "]" { // Don't error on just ']'
                        println!("poll_next: Unparseable remaining buffer at end: \"{}\"", self.buffer);
                        // This indicates malformed stream or unexpected ending.
                        // Optionally, return an error here instead of just ending.
                        // For now, we'll just clear the buffer and end the stream.
                        self.buffer.clear();
                    }

                    return Poll::Ready(None);
                }
                Poll::Pending => {
                    println!("poll_next: Inner stream pending.");
                    return Poll::Pending;
                },
            }
        }
    }
}