use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::core;
use crate::providers::openai::{
    Message,
    ReasoningEffort,
    Content,
    FunctionDefinition,
    ToolDefinition,
    FunctionCall,
    ToolCall,
};

/// Chat completion request.
///
/// API Reference: [Chat Completion](https://openrouter.ai/docs/api-reference/chat-completion)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Request {

    /// The model ID to use.
    pub model: String,

    /// A list of messages comprising the conversation so far.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<Message>,

    /// A list of tools the model may call.
    ///
    /// Currently, only functions are supported as a tool. Use this to
    /// provide a list of functions the model may generate JSON inputs
    /// for. A max of 128 functions are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,

    /// Alternate list of models for routing overrides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,

    /// Preferences for provider routing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<ProviderConfig>,

    /// Configuration for model reasoning/thinking tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,

    /// Whether to include usage information in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageConfig>,

    /// List of prompt transforms (OpenRouter-only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<Vec<String>>,

    /// Enable streaming of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Maximum number of tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,

    /// Sampling temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// Seed for deterministic outputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    /// Top-p sampling value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,

    /// Top-k sampling value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<f64>,

    /// Frequency penalty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penaly: Option<f64>,

    /// Presence penalty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penaly: Option<f64>,

    /// Repetition penalty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f64>,

    /// Mapping of token IDs to bias values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, f64>>,

    /// Number of top log probabilities to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i64>,

    /// Minimum probability threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f64>,

    /// Alternate top sampling parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_a: Option<f64>,

    /// A stable identifier for your end-users. Used to help detect
    /// and prevent abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Preferences for provider routing.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProviderConfig {

    /// Sort preference (e.g., price, throughput).
    pub sort: Option<String>,
}

/// Configuration for model reasoning/thinking tokens.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReasoningConfig {

    /// OpenAI-style reasoning effort setting.
    pub effort: Option<ReasoningEffort>,

    /// Non-OpenAI-style reasoning effort setting. Cannot be used
    /// simultaneously with effort.
    pub max_tokens: Option<i64>,

    /// Whether to exclude reasoning from the response.
    pub exclude: Option<bool>,
}

/// Whether to include usage information in the response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsageConfig {

    /// Whether to include usage information in the response.
    pub include: Option<bool>,
}

impl Request {

    /// Creates a [Request] from a single text prompt.
    pub fn from_prompt(model_id: &str, prompt: &str) -> Self {
        let mut req = Self::default();
        req.model = model_id.to_string();
        req.messages = vec![Message {
            role: Some(core::Role::User),
            content: Some(Content::Text(prompt.to_string())),
            tool_call_id: None,
            refusal: None,
            tool_calls: None,
        }];
        req
    }

    /// Creates a [Request] from a vector of messages.
    pub fn from_messages(model_id: &str, messages: Vec<Message>) -> Self {
        let mut req = Self::default();
        req.model = model_id.to_string();
        req.messages = messages;
        req
    }

    /// Specify a list of tools the model may call.
    pub fn with_tools(self, tools: Option<Vec<ToolDefinition>>) -> Self {
        Self { tools, ..self }
    }

    /// Specify configuration for model reasoning/thinking tokens.
    pub fn with_reasoning(self, reasoning: Option<ReasoningConfig>) -> Self {
        Self { reasoning, ..self }
    }

    /// Specify whether to include usage information in the response.
    pub fn with_usage(self, usage: Option<UsageConfig>) -> Self {
        Self { usage, ..self }
    }

    /// Specify whether to enable streaming of results.
    pub fn with_stream(self, stream: Option<bool>) -> Self {
        Self { stream, ..self }
    }

    /// Specify the number of maximum tokens.
    pub fn with_max_tokens(self, max_tokens: Option<i64>) -> Self {
        Self { max_tokens, ..self }
    }
}

impl From<core::Request> for Request {
    fn from(value: core::Request) -> Self {
        let mut messages = Vec::new();
        if let Some(system_prompt) = value.system {
            messages.push(Message {
                role: Some(core::Role::System),
                content: Some(Content::Text(system_prompt)),
                tool_call_id: None,
                refusal: None,
                tool_calls: None,
            });
        }

        let mut core_messages = value.messages.into_iter().peekable();
        while let Some(msg) = core_messages.next() {
            match msg.content {
                core::MessageContent::Text(text) => messages.push(Message {
                    role: Some(msg.role),
                    content: Some(Content::Text(text)),
                    tool_call_id: None,
                    refusal: None,
                    tool_calls: None,
                }),
                core::MessageContent::ToolCall(core_tool_call) => {
                    let mut tool_calls = vec![ToolCall {
                        id: Some(core_tool_call.id),
                        kind: Some("function".to_string()),
                        function: FunctionCall {
                            name: core_tool_call.name,
                            arguments: core_tool_call.arguments,
                        },
                    }];

                    while let Some(next_msg) = core_messages.peek() {
                        if let core::MessageContent::ToolCall(_) = &next_msg.content {
                            if let Some(core::Message { content: core::MessageContent::ToolCall(tc), .. }) = core_messages.next() {
                                tool_calls.push(ToolCall {
                                    id: Some(tc.id),
                                    kind: Some("function".to_string()),
                                    function: FunctionCall {
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
                        role: Some(core::Role::Assistant),
                        content: None,
                        tool_calls: Some(tool_calls),
                        tool_call_id: None,
                        refusal: None,
                    });
                },
                core::MessageContent::ToolResponse(tool_response) => messages.push(Message {
                    role: Some(core::Role::Tool),
                    tool_call_id: Some(tool_response.id),
                    content: Some(Content::Text(tool_response.content)),
                    refusal: None,
                    tool_calls: None,
                }),
            }
        }

        let tools = if value.tools.len() > 0 {
            Some(value.tools.into_iter().map(|t| ToolDefinition{
                kind: "function".to_string(),
                function: FunctionDefinition {
                    name: t.name,
                    description: Some(t.description),
                    parameters: t.parameters,
                    strict: None,
                },
            }).collect())
        } else {None};

        Self::from_messages(&value.model, messages)
            .with_max_tokens(value.max_tokens)
            .with_tools(tools)
            .with_usage(Some(UsageConfig{ include: Some(true) }))
    }
}
