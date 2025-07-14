use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "user")]
    User,

    #[serde(rename = "assistant")]
    Assistant,
}

/// A message to Claude.
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

/// MessageRequest is a request to the messages API.
///
/// See [Messages Reference](https://docs.anthropic.com/en/api/messages).
#[derive(Default, Serialize, Deserialize)]
pub struct MessageRequest {

    /// The model that will complete your prompt.
    ///
    /// See [API Reference](https://docs.anthropic.com/en/docs/models-overview) for additional details and options.
    pub model: String,

    /// Input messages.
    ///
    /// Anthropic models are trained to operate on alternating user
    /// and assistant conversational turns. When creating a new
    /// Message, you specify the prior conversational turns with the
    /// messages parameter, and the model then generates the next
    /// Message in the conversation. Consecutive user or assistant
    /// turns in your request will be combined into a single turn.
    pub messages: Vec<Message>,

    /// The maximum number of tokens to generate before stopping.
    ///
    /// Note that models may stop before reaching this maximum. This
    /// parameter only specifies the absolute maximum number of tokens
    /// to generate.
    pub max_tokens: i64,

    /// Whether to incrementally stream the response using server-sent
    /// events.
    pub stream: bool,

    /// System prompt.
    ///
    /// A system prompt is a way of providing context and instructions
    /// to Claude, such as specifying a particular goal or role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// Amount of randomness injected into the response.
    ///
    /// Defaults to 1.0. Ranges from 0.0 to 1.0. Use temperature closer to
    /// 0.0 for analytical / multiple choice, and closer to 1.0 for
    /// creative and generative tasks.
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<i64>,

    /// Configuration for enabling Claude's extended thinking.
    ///
    /// When enabled, responses include thinking content blocks
    /// showing Claude's thinking process before the final
    /// answer. Requires a minimum budget of 1,024 tokens and counts
    /// towards your max_tokens limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<Thinking>,

    /// How the model should use the provided tools.
    ///
    /// The model can use a specific tool, any available tool, decide
    /// by itself, or not use tools at all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Definitions of tools that the model may use.
    ///
    /// If you include tools in your API request, the model may return
    /// tool_use content blocks that represent the model's use of those
    /// tools. You can then run those tools using the tool input generated
    /// by the model and then optionally return results back to the model
    /// using tool_result content blocks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,

    /// Only sample from the top K options for each subsequent token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i64>,

    /// Use nucleus sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<i64>,
}

impl MessageRequest {

    /// Creates a [MessageRequest] from a single text prompt.
    pub fn from_prompt(model: &str, prompt: &str) -> Self {
        let mut req = Self::default();
        req.model = model.to_string();
        req.messages = vec![Message {
            role: Role::User,
            content: prompt.to_string(),
        }];
        req.max_tokens = 1024;
        req.stream = false;
        req
    }

    /// Creates a [MessageRequest] from a vector of messages.
    pub fn from_messages(model: &str, messages: Vec<Message>) -> Self {
        let mut req = Self::default();
        req.model = model.to_string();
        req.messages = messages;
        req.max_tokens = 1024;
        req.stream = false;
        req
    }

    /// Populates the `max_tokens` field with the given value.
    pub fn with_max_tokens(self, max_tokens: i64) -> Self {
        Self { max_tokens, ..self }
    }

    /// Populates the `stream` field with the given value.
    pub fn with_stream(self, stream: bool) -> Self {
        Self { stream, ..self }
    }

    /// Populates the `reasoning` field with the given value.
    pub fn with_system(self, system: Option<String>) -> Self {
        Self { system, ..self }
    }

    /// Populates the `temperature` field with the given value.
    pub fn with_temperature(self, temperature: Option<i64>) -> Self {
        Self { temperature, ..self }
    }

    /// Populates the `thinking` field with the given value.
    pub fn with_thinking(self, thinking: Option<Thinking>) -> Self {
        Self { thinking, ..self }
    }

    /// Populates the `tool_choice` field with the given value.
    pub fn with_tool_choice(self, tool_choice: Option<ToolChoice>) -> Self {
        Self { tool_choice, ..self }
    }

    /// Populates the `tools` field with the given value.
    pub fn with_tools(self, tools: Option<Vec<ToolDefinition>>) -> Self {
        Self { tools, ..self }
    }

    /// Populates the `top_k` field with the given value.
    pub fn with_top_k(self, top_k: Option<i64>) -> Self {
        Self { top_k, ..self }
    }

    /// Populates the `top_p` field with the given value.
    pub fn with_top_p(self, top_p: Option<i64>) -> Self {
        Self { top_p, ..self }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ThinkingKind {
    #[serde(rename="enabled")]
    Enabled,
    #[serde(rename="disabled")]
    Disabled,
}

/// Configuration for enabling Claude's extended thinking.
#[derive(Serialize, Deserialize)]
pub struct Thinking {
    #[serde(rename = "type")]
    pub kind: ThinkingKind,

    /// Determines how many tokens Claude can use for its internal
    /// reasoning process. Larger budgets can enable more thorough
    /// analysis for complex problems, improving response quality.
    ///
    ///
    /// Must be ≥1024 and less than [MessageRequest::max_tokens].
    pub budget_tokens: Option<i64>,
}

/// The type of tool choice configuration.
#[derive(Serialize, Deserialize)]
pub enum ToolChoiceKind {

    /// The model can decide by itself.
    #[serde(rename = "auto")]
    Auto,

    /// Use any available tool.
    #[serde(rename = "any")]
    Any,

    /// Use a specific tool.
    #[serde(rename = "tool")]
    Tool,

    /// Do not use tools.
    #[serde(rename = "none")]
    None
}

/// How the model should use the provided tools.
#[derive(Serialize, Deserialize)]
pub struct ToolChoice {
    pub kind: ToolChoiceKind,

    /// Whether to disable parallel tool use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_parallel_tool_use: Option<bool>,

    /// The name of the tool to use.
    ///
    /// Only use when kind is set to [ToolChoiceKind::Tool].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Define a tool function.
#[derive(Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

/// Response from the [message](super::Client::messages)
/// route. Contains the LLMs response, including tool use, web search,
/// and textual results.
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {

    /// Unique object identifier.
    pub id: String,

    /// Conversational role of the generated message.
    pub role: Role,

    /// Content generated by the model.
    ///
    /// This is an array of content blocks, each of which has a type
    /// that determines its shape.
    pub content: Vec<ContentResponse>,

    /// The model that handled the request.
    pub model: String,

    /// The reason the response stopped.
    pub stop_reason: Option<StopReason>,

    /// Billing and rate-limit usage.
    pub usage: Usage,

}

/// Enumerates the possible content types of a [MessageResponse].
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentResponse {
    Text(TextResponse),
    Thinking(ThinkingResponse),
    RedactedThinkng(RedactedThinkingResponse),
    ToolUse(ToolUseResponse),
    WebSearchTool(WebSearchToolResponse),
}

/// Contains a standard textual LLM response.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextResponse {
    #[serde(rename = "type")]
    pub kind: String,
    pub text: String,
}

/// Contains an LLMs thinking trace.
#[derive(Debug, Serialize, Deserialize)]
pub struct ThinkingResponse {
    #[serde(rename = "type")]
    pub kind: String,
    pub thinking: String,
    pub signature: String,
}

/// Signals that the LLM is thinking, but the thinking trace is
/// redacted.
#[derive(Debug, Serialize, Deserialize)]
pub struct RedactedThinkingResponse {
    #[serde(rename = "type")]
    pub kind: String,
    pub data: String,
}

/// Reports on the call of a custom tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolUseResponse {
    #[serde(rename = "type")]
    pub kind: String,
    pub input: String,
    pub name: String,
    pub id: String,
}

/// Contains the response of the web search server tool, which may
/// contain multiple fetched web pages.
#[derive(Debug, Serialize, Deserialize)]
pub struct WebSearchToolResponse {
    #[serde(rename = "type")]
    pub kind: String,
    pub tool_use_id: String,
    pub content: Option<Vec<WebSearchContent>>,
}

/// Contains the content of a single web page returned from the web
/// search server tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct WebSearchContent {
    pub encrypted_content: String,
    pub page_age: Option<String>,
    pub title: String,
    pub url: String,
}

/// Reports on the reason the LLM stopped generating tokens.
#[derive(Debug, Serialize, Deserialize)]
pub enum StopReason {

    /// The model reached a natural stopping point.
    #[serde(rename = "end_turn")]
    EndTurn,

    /// Exceeded the requested max_tokens or the model's maximum.
    #[serde(rename = "max_tokens")]
    MaxTokens,

    /// One of the custom `stop_sequences` was generated.
    #[serde(rename = "stop_sequence")]
    StopSequence,

    /// The model invoked one or more tools.
    #[serde(rename = "tool_use")]
    ToolUse,

    /// Paused a long-running turn.
    #[serde(rename = "pause_turn")]
    PauseTurn,

    /// When streaming classifiers intervene to handle potential
    /// policy violations.
    #[serde(rename = "refusal")]
    Refusal,
}

/// Billing and rate-limit usage.
#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {

    /// The number of input tokens which were used.
    pub input_tokens: Option<i64>,

    /// The number of output tokens which were used.
    pub output_tokens: Option<i64>,

    /// If the request used the priority, standard, or batch tier.
    pub service_tier: Option<ServiceTier>,
}

/// Anthropic offers different tiers of service. The `ServiceTier`
/// enum reports on the service tier used to satisfy a
/// [MessageRequest].
///
/// API Reference: [Service Tiers](https://docs.anthropic.com/en/api/service-tiers)
#[derive(Debug, Serialize, Deserialize)]
pub enum ServiceTier {

    /// Default tier for both piloting and scaling everyday use cases.
    #[serde(rename = "standard")]
    Standard,

    /// Best for workflows deployed in production where time,
    /// availability, and predictable pricing are important
    #[serde(rename = "priority")]
    Priority,

    /// Best for asynchronous workflows which can wait or benefit from
    /// being outside your normal capacity.
    #[serde(rename = "batch")]
    Batch,
}
