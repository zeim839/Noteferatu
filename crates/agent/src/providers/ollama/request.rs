use serde::{Serialize, Deserialize};
use crate::core::{Role, ToolDefinition, MessageContent};
use super::message::*;
use crate::core;

/// Ollama chat completion request.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Request {

    /// The model ID to use.
    pub model: String,

    /// A list of messages comprising the conversation so far.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<Message>,

    /// A list of tools that the model may call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,

    /// Enable thinking mode (available only for thinking models).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<bool>,

    /// Keep model alive for the specified duration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<f64>,

    /// Whether to stream the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl Request {

    /// Creates a [Request] from a single text prompt.
    pub fn from_prompt(model: &str, prompt: &str) -> Self {
        let mut req = Self::default();
        req.model = model.to_string();
        req.messages = vec![Message {
            role: Role::User,
            content: Some(prompt.to_string()),
            tool_calls: None,
            images: None,
            thinking: None,
        }];
        req
    }

    /// Creates a [Request] from a vector of messages.
    pub fn from_messages(model: &str, messages: Vec<Message>) -> Self {
        let mut req = Self::default();
        req.model = model.to_string();
        req.messages = messages;
        req
    }

    /// Populates the [Self::tools] field with the given value.
    pub fn with_tools(self, tools: Option<Vec<ToolDefinition>>) -> Self {
        Self { tools, ..self }
    }

    /// Populates the [Self::think] field with the given value.
    pub fn with_think(self, think: Option<bool>) -> Self {
        Self { think, ..self }
    }

    /// Populates the [Self::keep_alive] field with the given value.
    pub fn with_keep_alive(self, keep_alive: Option<f64>) -> Self {
        Self { keep_alive, ..self }
    }

    /// Populates the [Self::stream] field with the given value.
    pub fn with_stream(self, stream: Option<bool>) -> Self {
        Self { stream, ..self }
    }
}

impl From<core::Request> for Request {
    fn from(value: core::Request) -> Self {
        let mut messages = Vec::new();
        if let Some(system_prompt) = value.system {
            messages.push(Message {
                role: Role::System,
                content: Some(system_prompt),
                tool_calls: None,
                images: None,
                thinking: None,
            });
        }

        let mut core_messages = value.messages.into_iter().peekable();
        while let Some(msg) = core_messages.next() {
            match msg.content {
                MessageContent::Text(text) => messages.push(Message {
                    role: msg.role,
                    content: Some(text),
                    tool_calls: None,
                    images: None,
                    thinking: None,
                }),
                MessageContent::ToolCall(tool_call) => {
                    let mut tool_calls = vec![tool_call];
                    // Group consecutive tool calls into a single message.
                    while let Some(next_msg) = core_messages.peek() {
                        if let MessageContent::ToolCall(_) = &next_msg.content {
                            if let Some(core::Message { content: MessageContent::ToolCall(tc), .. }) = core_messages.next() {
                                tool_calls.push(tc);
                            }
                        } else {
                            break;
                        }
                    }
                    messages.push(Message {
                        role: Role::Assistant,
                        content: None,
                        tool_calls: Some(tool_calls),
                        images: None,
                        thinking: None,
                    });
                }
                MessageContent::ToolResponse(tool_response) => messages.push(Message {
                    role: Role::Tool,
                    content: Some(tool_response.content),
                    tool_calls: None,
                    images: None,
                    thinking: None,
                }),
            }
        }
        Request {
            model: value.model,
            messages,
            tools: if value.tools.is_empty() { None } else { Some(value.tools) },
            ..Default::default()
        }
    }
}
