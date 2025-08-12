use serde::{Serialize, Deserialize};
use super::message::*;
use crate::core;

/// A response to a completion [Request](super::Request).
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {

    /// The model that responded to the request.
    pub model: String,

    /// The response message.
    pub message: Message,

    /// The reason the LLM was terminated.
    pub done_reason: Option<String>,

    /// Whether the model is done responding to the request.
    pub done: bool,

    /// Tokens in the prompt.
    pub prompt_eval_count: Option<i64>,

    /// Total token count.
    pub eval_count: Option<i64>,
}

impl Into<core::Response> for Response {
    fn into(self) -> core::Response {
        let mut messages = Vec::new();

        if let Some(content) = self.message.content {
            // Ollama may return an empty content string when tool calls are present.
            if !content.is_empty() {
                messages.push(core::Message {
                    role: self.message.role.clone(),
                    content: core::MessageContent::Text(content),
                });
            }
        }

        if let Some(tool_calls) = self.message.tool_calls {
            messages.extend(tool_calls.into_iter().map(|tc| core::Message {
                role: self.message.role.clone(),
                content: core::MessageContent::ToolCall(tc),
            }));
        }

        let prompt_tokens = self.prompt_eval_count.unwrap_or(0);
        let completion_tokens = self.eval_count.unwrap_or(0);

        core::Response {
            messages,
            usage: core::Usage {
                prompt_tokens,
                completion_tokens,
                total_tokens: prompt_tokens + completion_tokens,
            },
            error: None,
        }
    }
}
