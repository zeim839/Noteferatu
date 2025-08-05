use serde::{Serialize, Deserialize};
use super::message::*;
use crate::core;

/// MessageRequest is a request to the messages API.
///
/// See [Messages Reference](https://docs.anthropic.com/en/api/messages).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Request {

    /// The model that will complete your prompt.
    ///
    /// See [API Reference](https://docs.anthropic.com/en/docs/models-overview) for additional details and options.
    pub model: String,

    /// Input messages.
    pub messages: Vec<Message>,

    /// The maximum number of tokens to generate before stopping.
    ///
    /// Note that models may stop before reaching this maximum. This
    /// parameter only specifies the absolute maximum number of tokens
    /// to generate.
    pub max_tokens: i64,

    /// Use Server-sent-events to stream the response.
    pub stream: bool,

    /// System prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// Definitions of tools that the model may use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
}

impl Request {

    /// Creates a [Request] from a single text prompt.
    pub fn from_prompt(model: &str, prompt: &str) -> Self {
        let mut req = Self::default();
        req.model = model.to_string();
        req.messages = vec![Message {
            role: core::Role::User,
            content: vec![Content{
                kind: ContentKind::Text,
                text: Some(prompt.to_string()),
                ..Default::default()
            }],
        }];
        req.max_tokens = 1024;
        req.stream = false;
        req
    }

    /// Creates a [Request] from a vector of messages.
    pub fn from_messages(model: &str, messages: Vec<Message>) -> Self {
        let mut req = Self::default();
        req.model = model.to_string();
        req.messages = messages;
        req.max_tokens = 1024;
        req.stream = false;
        req
    }

    /// Populates the [Self::max_tokens] field with the given value.
    pub fn with_max_tokens(self, max_tokens: i64) -> Self {
        Self { max_tokens, ..self }
    }

    /// Populates the [Self::stream] field with the given value.
    pub fn with_stream(self, stream: bool) -> Self {
        Self { stream, ..self }
    }

    /// Populates the [Self::system] field with the given value.
    pub fn with_system(self, system: Option<String>) -> Self {
        Self { system, ..self }
    }
}

impl From<core::Request> for Request {
    fn from(value: core::Request) -> Self {
        let tools = if value.tools.is_empty() {
            None
        } else {
            Some(value.tools.into_iter().map(Into::into).collect())
        };

        Self {
            model: value.model,
            messages: value.messages.into_iter().map(Into::into).collect(),
            max_tokens: value.max_tokens.unwrap_or(4096),
            stream: false,
            system: value.system,
            tools,
        }
    }
}

/// Define a tool function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {

    /// Function name.
    pub name: String,

    /// Description of what the function does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// JSON schema for the function's arguments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<serde_json::Value>,
}

impl From<core::ToolDefinition> for ToolDefinition {
    fn from(value: core::ToolDefinition) -> Self {
        Self {
            name: value.name,
            description: Some(value.description),
            input_schema: value.parameters,
        }
    }
}