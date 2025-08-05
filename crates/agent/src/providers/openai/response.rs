use serde::{Serialize, Deserialize};
use crate::core::{Usage, self};
use super::message::*;

/// Response to a chat completion [Request](super::Request).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {

    /// A list of chat completion choices.
    pub choices: Vec<Choice>,

    /// The Unix timestamp (in seconds) of when the chat completion
    /// was created.
    pub created: i64,

    /// A unique identifier for the chat completion.
    pub id: String,

    /// The model used for the chat completion.
    pub model: String,

    /// Usage statistics for the completion request.
    pub usage: Option<Usage>,
}

impl Into<core::Response> for Response {
    fn into(self) -> core::Response {
        let mut messages = Vec::new();

        for choice in self.choices {
            // A choice will have either a `message` (for non-streaming) or a `delta` (for streaming).
            let message = choice.message.or(choice.delta);

            if let Some(msg) = message {
                let role = msg.role.unwrap_or(core::Role::Assistant);

                // A message from the assistant can contain both text and tool calls.
                // The text content might be in `content`.
                if let Some(content) = msg.content {
                    match content {
                        Content::Text(text) => {
                            // In streaming, an empty text content can be sent.
                            // We only create a message if there is text.
                            if !text.is_empty() {
                                messages.push(core::Message {
                                    role: role.clone(),
                                    content: core::MessageContent::Text(text),
                                });
                            }
                        }
                        // The core::Message doesn't support multimodal content yet.
                        Content::Content(_) => {}
                    }
                }

                // The tool calls are in `tool_calls`.
                if let Some(tool_calls) = msg.tool_calls {
                    for tool_call in tool_calls {
                        messages.push(core::Message {
                            role: role.clone(),
                            content: core::MessageContent::ToolCall(core::ToolCall {
                                // In streaming, the id can be None initially, but we need an id.
                                id: tool_call.id.unwrap_or_default(),
                                name: tool_call.function.name,
                                // Arguments can be a partial JSON string during streaming.
                                // It's up to the consumer to handle this.
                                arguments: tool_call.function.arguments,
                            }),
                        });
                    }
                }
            }
        }

        core::Response {
            messages,
            usage: self.usage.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {

    /// The reason the model stopped generating tokens.
    pub finish_reason: Option<FinishReason>,

    /// The index of the choice in the list of choices.
    pub index: i64,

    /// The index of the choice in the list of choices.
    pub message: Option<Message>,

    /// Change since the last choice output (used for streaming
    /// completions).
    pub delta: Option<Message>,
}

/// The reason the model stopped generating tokens.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {

    /// `stop` if the model hit a natural stop point or a provided
    /// stop sequence.
    Stop,

    /// `length` if the maximum number of tokens specified in the
    /// request was reached.
    Length,

    /// `content_filter` if content was omitted due to a flag from our
    /// content filters
    ContentFilter,

    /// `tool_calls` if the model called a tool.
    ToolCalls,

    /// `function_call` (deprecated) if the model called a function.
    FunctionCall,
}
