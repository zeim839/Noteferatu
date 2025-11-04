//! Chat generation response.

use serde::Deserialize;
use super::message::Message;

/// Chat completion response.
///
/// See: [API Reference](https://docs.ollama.com/api/chat#response-model)
#[derive(Deserialize, Debug, Clone)]
pub struct Response {

    /// Model name used to generate this message.
    pub model: String,

    /// Timestamp of response creation (ISO 8601).
    pub created_at: String,

    /// Message response.
    pub message: Message,

    /// Indicates whether the chat response has finished.
    pub done: bool,

    /// Reason the response finished.
    pub done_reason: Option<String>,

    /// Total time spent generating in nanoseconds.
    pub total_duration: Option<u64>,

    /// Time spent loading the model in nanoseconds.
    pub load_duration: Option<u64>,

    /// Number of tokens in the prompt.
    pub prompt_eval_count: Option<u64>,

    /// Time spent evaluating the prompt in nanoseconds.
    pub prompt_eval_duration: Option<u64>,

    /// Number of tokens generated in the response.
    pub eval_count: Option<u64>,

    /// Time spent generating tokens in nanoseconds.
    pub eval_duration: Option<u64>,
}
