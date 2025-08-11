use serde::{Serialize, Deserialize};
use crate::core::{self, MessageContent, Role};
use super::message::*;

/// Chat completion request.
///
/// API Reference: [Create Chat Completion](https://platform.openai.com/docs/api-reference/chat/create)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Request {

    /// A list of messages comprising the conversation so
    /// far. Depending on the model you use, different message types
    /// (modalities) are supported, like text, images, and audio.
    pub messages: Vec<Message>,

    /// The model ID to use.
    pub model: String,

    /// An upper bound for the number of tokens that can be generated
    /// for a completion, including visible output tokens and
    /// reasoning tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i64>,

    /// If set to true, the model response data will be streamed to
    /// the client as it is generated using server-sent events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// A list of tools the model may call.
    ///
    /// Currently, only functions are supported as a tool. Use this to
    /// provide a list of functions the model may generate JSON inputs
    /// for. A max of 128 functions are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,

    /// Whether a model's response may include multiple tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
}

/// Reasoning effort setting.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    Low,
    #[default]
    Medium,
    High,
}

impl Request {

    /// Creates a [Request] from a single text prompt.
    pub fn from_prompt(model: &str, prompt: &str) -> Self {
        let mut req = Self::default();
        req.model = model.to_string();
        req.messages = vec![Message {
            role: Some(Role::User),
            content: Some(Content::Text(prompt.to_string())),
            tool_call_id: None,
            refusal: None,
            tool_calls: None,
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

    /// Populates the [Self::max_completion_tokens] field with the given value.
    pub fn with_max_completion_tokens(self, max_completion_tokens: Option<i64>) -> Self {
        Self { max_completion_tokens, ..self }
    }

    /// Populates the [Self::parallel_tool_calls] field with the given value.
    pub fn with_parallel_tool_calls(self, parallel_tool_calls: Option<bool>) -> Self {
        Self { parallel_tool_calls, ..self }
    }

    /// Populates the [Self::stream] field with the given value.
    pub fn with_stream(self, stream: Option<bool>) -> Self {
        Self { stream, ..self }
    }

    /// Populates the [Self::tools] field with the given value.
    pub fn with_tools(self, tools: Option<Vec<ToolDefinition>>) -> Self {
        Self { tools, ..self }
    }
}

impl From<core::Request> for Request {
    fn from(value: core::Request) -> Self {
        let mut messages = Vec::new();
        if let Some(system_prompt) = value.system {
            messages.push(Message {
                role: Some(Role::System),
                content: Some(Content::Text(system_prompt)),
                tool_call_id: None,
                refusal: None,
                tool_calls: None,
            });
        }

        let mut core_messages = value.messages.into_iter().peekable();
        while let Some(msg) = core_messages.next() {
            match msg.content {
                MessageContent::Text(text) => messages.push(Message {
                    role: Some(msg.role),
                    content: Some(Content::Text(text)),
                    tool_call_id: None,
                    refusal: None,
                    tool_calls: None,
                }),
                MessageContent::ToolCall(core_tool_call) => {
                    let mut tool_calls = vec![super::message::ToolCall {
                        id: Some(core_tool_call.id),
                        kind: Some("function".to_string()),
                        function: super::message::FunctionCall {
                            name: core_tool_call.name,
                            arguments: core_tool_call.arguments,
                        },
                    }];

                    while let Some(next_msg) = core_messages.peek() {
                        if let MessageContent::ToolCall(_) = &next_msg.content {
                            if let Some(core::Message { content: MessageContent::ToolCall(tc), .. }) = core_messages.next() {
                                tool_calls.push(super::message::ToolCall {
                                    id: Some(tc.id),
                                    kind: Some("function".to_string()),
                                    function: super::message::FunctionCall {
                                        name: tc.name,
                                        arguments: tc.arguments,
                                    },
                                });
                            }
                        } else {
                            break;
                        }
                    }

                    messages.push(Message {
                        role: Some(Role::Assistant),
                        content: None,
                        tool_calls: Some(tool_calls),
                        tool_call_id: None,
                        refusal: None,
                    });
                }
                MessageContent::ToolResponse(tool_response) => messages.push(Message {
                    role: Some(Role::Tool),
                    tool_call_id: Some(tool_response.id),
                    content: Some(Content::Text(tool_response.content)),
                    refusal: None,
                    tool_calls: None,
                }),
            }
        }

        let tools = if value.tools.len() > 0 {
            Some(value.tools.into_iter().map(|t| ToolDefinition {
                kind: "function".to_string(),
                function: FunctionDefinition {
                    name: t.name,
                    description: Some(t.description),
                    parameters: t.parameters,
                    strict: None,
                },
            }).collect())
        } else { None };

        Self {
            model: value.model,
            messages,
            max_completion_tokens: value.max_tokens,
            tools,
            stream: None,
            parallel_tool_calls: None,
        }
    }
}
