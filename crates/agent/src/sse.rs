use super::error::Error;
use std::fmt::Display;
use core::fmt::Debug;
use tokio_stream::{Stream, StreamExt};
use std::pin::Pin;
use bytes::Bytes;

/// Implements Server-Sent Events streaming for LLM responses.
pub struct SSE<I> {
    inner: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    parser: Box<dyn Fn(&mut String) -> Option<I>>,
}

impl<I> SSE<I> {

    /// Create a new [SSE] instance.
    pub(crate) fn new(
        parser: Box<dyn Fn(&mut String) -> Option<I>>,
        stream: impl Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static
    ) -> Self {
        Self {inner: Box::pin(stream), parser}
    }

    /// Get the next SSE event.
    pub async fn next<E: Display + Debug>(&mut self) -> Option<Result<I, Error<E>>> {
        let mut buffer = String::new();
        while let Some(chunk) = self.inner.next().await {
            match chunk {
                Ok(bytes) => {
                    buffer.push_str(&String::from_utf8_lossy(&bytes));
                    match (self.parser)(&mut buffer) {
                        Some(item) => return Some(Ok(item)),
                        None => continue,
                    }
                },
                Err(e) => return Some(Err(Error::Http(e))),
            }
        }
        match (self.parser)(&mut buffer) {
            Some(item) => return Some(Ok(item)),
            None => return None,
        }
    }
}
