//! Chat completion request.

use super::tools::{ToolChoiceConfig, ToolChoiceMode, ToolCallChoice};
use super::message::Message;
use serde::Serialize;

/// Chat completion request.
///
/// See: [API Reference](https://openrouter.ai/docs/api-reference/chat/send-chat-completion-request#request)
#[derive(Serialize, Debug, Clone, Default)]
pub struct Request {

    /// Messages comprising the conversation so far.
    pub messages: Vec<Message>,

    /// The model to use to generate the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// A list of models to select from.
    ///
    /// If one model is unavailable, the next one is used, etc.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub models: Vec<String>,

    /// Positive values penalize new tokens based on their existing
    /// frequency in the text so far, decreasing the model's
    /// likelihood to repeat the same line verbatim.
    ///
    /// Number between -2.0 and 2.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,

    /// Sample the `top_logprobs` number of highest output
    /// probabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u64>,

    /// Set the maximum completion tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u64>,

    /// Positive values penalize new tokens based on whether they
    /// appear in the text so far, increasing the model's likelihood
    /// to talk about new topics.
    ///
    /// Number between -2.0 and 2.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,

    /// Internal reasoning configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,

    /// Generation seed for pseudo-deterministic outputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,

    /// Whether to stream the output.
    pub stream: bool,

    /// Streaming configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,

    /// What sampling temperature to use.
    ///
    /// Number between 0 and 2. Higher values like 0.8 will make the
    /// output more random, while lower values like 0.2 will make it
    /// more focused and deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// Configure how the model should call tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoiceConfig>,

    /// A list of tools the model may call.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<serde_json::Value>,

    /// An alternative to sampling with temperature, called nucleus
    /// sampling, where the model considers the results of the tokens
    /// with top_p probability mass. So 0.1 means only the tokens
    /// comprising the top 10% probability mass are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,

    /// Optional user identification string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Request {

    /// Create a [Request] by specifying a model to use.
    pub fn from_model(model: &str) -> Self {
        let mut req = Self::default();
        req.model = Some(model.to_string());
        req
    }

    /// Create a [Request] by specifying a list of models to use.
    pub fn from_models(models: Vec<&str>) -> Self {
        let mut req = Self::default();
        req.models = models.into_iter().map(|v| v.to_string()).collect();
        req
    }

    /// Push a model to the [Request]s model choices.
    pub fn push_model(self, model: &str) -> Self {
        let mut req = self;
        req.models.push(model.to_string());
        req
    }

    /// Push a message to the [Request] conversation.
    pub fn push_message(self, message: Message) -> Self {
        let mut req = self;
        req.messages.push(message);
        req
    }

    /// Append a list of messages to the [Request] conversation.
    ///
    /// Consumes `messages`.
    pub fn append_message(self, messages: &mut Vec<Message>) -> Self {
        let mut req = self;
        req.messages.append(messages);
        req
    }

    /// Append a list of models that the [Request] may use.
    ///
    /// Consumes `models`.
    pub fn append_models(self, models: &mut Vec<String>) -> Self {
        let mut req = self;
        req.models.append(models);
        req
    }

    /// Set a frequency penalty.
    pub fn frequency_penalty(self, penalty: f64) -> Self {
        Self { frequency_penalty: Some(penalty), ..self }
    }

    /// Set the top_logprobs sampling parameter.
    pub fn top_logprobs(self, top_logprobs: u64) -> Self {
        Self { top_logprobs: Some(top_logprobs), ..self }
    }

    /// Set the max completion tokens to generate.
    pub fn max_completion_tokens(self, tokens: u64) -> Self {
        Self { max_completion_tokens: Some(tokens), ..self }
    }

    /// Set a presence penalty.
    pub fn presence_penalty(self, penalty: f64) -> Self {
        Self { presence_penalty: Some(penalty), ..self }
    }

    /// Set a reasoning config.
    ///
    /// Overrides [reasoning_effort] and [reasoning_summary].
    pub fn reasoning(self, config: ReasoningConfig) -> Self {
        Self { reasoning: Some(config), ..self }
    }

    /// Set the reasoning effort config.
    pub fn reasoning_effort(self, effort: ReasoningEffort) -> Self {
        let mut req = self;
        match &mut req.reasoning {
            Some(config) => {
                config.effort = Some(effort);
            },
            None => {
                req.reasoning = Some(ReasoningConfig {
                    effort: Some(effort),
                    summary: None,
                });
            }
        }
        req
    }

    /// Set the reasoning summary config.
    pub fn reasoning_summary(self, summary: ReasoningSummary) -> Self {
        let mut req = self;
        match &mut req.reasoning {
            Some(config) => {
                config.summary = Some(summary);
            },
            None => {
                req.reasoning = Some(ReasoningConfig {
                    effort: None,
                    summary: Some(summary),
                });
            }
        }
        req
    }

    /// Set the generation seed.
    pub fn seed(self, seed: u64) -> Self {
        Self { seed: Some(seed), ..self }
    }

    /// Specify whether to stream the response.
    pub fn stream(self, stream: bool) -> Self {
        Self { stream, ..self }
    }

    /// Set streaming options.
    pub fn stream_options(self, include_usage: bool) -> Self {
        Self { stream_options: Some(StreamOptions{
            include_usage: Some(include_usage),
        }), ..self}
    }

    /// Set the sampling temperature.
    pub fn temperature(self, temp: f64) -> Self {
        Self { temperature: Some(temp), ..self }
    }

    /// Set the tool choice configuration.
    ///
    /// Overrides [tool_choice_mode] and [tool_choice_object].
    pub fn tool_choice(self, config: ToolChoiceConfig) -> Self {
        Self { tool_choice: Some(config), ..self }
    }

    /// Set the tool choice mode.
    pub fn tool_choice_mode(self, mode: ToolChoiceMode) -> Self {
        let mut req = self;
        req.tool_choice = Some(ToolChoiceConfig::from(mode));
        req
    }

    /// Set a function to call as the [tool_choice] config.
    pub fn tool_choice_object(self, object: ToolCallChoice) -> Self {
        let mut req = self;
        req.tool_choice = Some(ToolChoiceConfig::Object(object));
        req
    }

    /// Push a tool to the list of tools the model may call from.
    pub fn push_tool(self, tool: serde_json::Value) -> Self {
        let mut req = self;
        req.tools.push(tool);
        req
    }

    /// Append a list of tools to the list of tools the model may call
    /// from.
    ///
    /// Consumes `tools`.
    pub fn append_tools(self, tools: &mut Vec<serde_json::Value>) -> Self {
        let mut req = self;
        req.tools.append(tools);
        req
    }

    /// Set the `top_p` sampling parameter.
    pub fn top_p(self, top_p: f64) -> Self {
        Self { top_p: Some(top_p), ..self }
    }

    /// Set a user string.
    pub fn user(self, user: &str) -> Self {
        Self { user: Some(user.to_string()), ..self }
    }
}

/// Configures the model's internal reasoning.
#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct ReasoningConfig {
    pub effort: Option<ReasoningEffort>,
    pub summary: Option<ReasoningSummary>,
}

impl From<ReasoningEffort> for ReasoningConfig {
    fn from(value: ReasoningEffort) -> Self {
        Self { effort: Some(value), summary: None }
    }
}

impl From<ReasoningSummary> for ReasoningConfig {
    fn from(value: ReasoningSummary) -> Self {
        Self { summary: Some(value), effort: None }
    }
}

/// Specifies the token length and time spent reasoning.
///
/// Part of [ReasoningConfig]
#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    Minimal,
    Low,
    Medium,
    High,
}

/// How the model should output reasoning summaries.
#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningSummary {
    Auto,
    Concise,
    Detailed,
}

/// Streaming configuration.
#[derive(Serialize, Debug, Clone)]
pub struct StreamOptions {

    /// Whether to include usage information in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}
