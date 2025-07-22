use serde::{Serialize, Deserialize};
use crate::openai::{ErrorAPI, FunctionDefinition};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
}

/// A message to Claude.
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Vec<Content>,
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

    /// Amount of randomness injected into the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,

    /// Only sample from the top K options for each subsequent token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<f64>,

    /// Use nucleus sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
}

impl MessageRequest {

    /// Creates a [MessageRequest] from a single text prompt.
    pub fn from_prompt(model: &str, prompt: &str) -> Self {
        let mut req = Self::default();
        req.model = model.to_string();
        req.messages = vec![Message {
            role: Role::User,
            content: vec![Content{
                kind: ContentKind::Text,
                text: Some(prompt.to_string()),
                thinking: None,
                signature: None,
                data: None,
                input: None,
                name: None,
                tool_use_id: None,
            }],
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

    /// Populates the [Self::temperature] field with the given value.
    pub fn with_temperature(self, temperature: Option<f64>) -> Self {
        Self { temperature, ..self }
    }

    /// Populates the [Self::thinking] field with the given value.
    pub fn with_thinking(self, thinking: Option<Thinking>) -> Self {
        Self { thinking, ..self }
    }

    /// Populates the [Self::tool_choice] field with the given value.
    pub fn with_tool_choice(self, tool_choice: Option<ToolChoice>) -> Self {
        Self { tool_choice, ..self }
    }

    /// Populates the [Self::tools] field with the given value.
    pub fn with_tools(self, tools: Option<Vec<ToolDefinition>>) -> Self {
        Self { tools, ..self }
    }

    /// Populates the [Self::top_k] field with the given value.
    pub fn with_top_k(self, top_k: Option<f64>) -> Self {
        Self { top_k, ..self }
    }

    /// Populates the [Self::top_p] field with the given value.
    pub fn with_top_p(self, top_p: Option<f64>) -> Self {
        Self { top_p, ..self }
    }
}

impl crate::Request for MessageRequest {

    /// Populates the [Self::max_tokens] field with the given value.
    fn with_max_tokens(self, max_tokens: Option<i64>) -> Self {
        if max_tokens.is_none() { self } else {
            Self { max_tokens: max_tokens.unwrap_or(1024), ..self }
        }
    }

    /// Populates the [Self::temperature] field with the given value.
    fn with_temperature(self, temperature: Option<f64>) -> Self {
        Self { temperature, ..self }
    }

    fn with_web_search_results(self, web_search_results: Option<i64>) -> Self {
        let mut tools = self.tools.unwrap_or_default();
        tools.push(ToolDefinition{
            name: "web_search".to_string(),
            kind: Some("web_search_20250305".to_string()),
            max_uses: web_search_results.map(|v| v as u64),
            input_schema: None,
            description: None,
        });
        Self { tools: Some(tools), ..self }
    }

    fn with_tools(self, tools: Option<Vec<FunctionDefinition>>) -> Self {
        let tools = tools.map(|v| v.iter().map(|item| {
            ToolDefinition{
                name: item.name.clone(),
                description: item.description.clone(),
                input_schema: item.parameters.clone(),
                kind: None,
                max_uses: None,
            }
        }).collect());
        Self { tools, ..self }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThinkingKind {
    Enabled,
    Disabled,
}

/// Configuration for enabling Claude's extended thinking.
#[derive(Serialize, Deserialize)]
pub struct Thinking {

    /// The type of tool choice configuration.
    #[serde(rename = "type")]
    pub kind: ThinkingKind,

    /// Determines how many tokens Claude can use for its internal
    /// reasoning process. Larger budgets can enable more thorough
    /// analysis for complex problems, improving response quality.
    ///
    ///
    /// Must be ≥1024 and less than [MessageRequest::max_tokens].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_tokens: Option<i64>,
}

/// The type of tool choice configuration.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoiceKind {
    /// The model can decide by itself.
    Auto,

    /// Use any available tool.
    Any,

    /// Use a specific tool.
    Tool,

    /// Do not use tools.
    None
}

/// How the model should use the provided tools.
#[derive(Serialize, Deserialize)]
pub struct ToolChoice {
    #[serde(rename = "type")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<serde_json::Value>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Only applies to the `web_search` server tool: the maximum
    /// number of web sources to fetch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<u64>,
}

/// Response from the [completion](super::Client::completion)
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
    pub content: Vec<Content>,

    /// The model that handled the request.
    pub model: String,

    /// The reason the response stopped.
    pub stop_reason: Option<StopReason>,

    /// Billing and rate-limit usage.
    pub usage: Usage,

}

/// Enumerates the possible types of streaming response events.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamEventType {
    Error,
    MessageStart,
    ContentBlockStart,
    ContentBlockDelta,
    ContentBlockStop,
    MessageDelta,
    MessageStop,
    Ping,
    Other(String),
}

/// An Server-Sent-Event message response from the
/// [completion](super::Client::completion) route.
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamResponse {

    /// The type of event.
    #[serde(rename = "type")]
    pub kind: StreamEventType,

    /// Possible error body for [StreamEventType::Error] responses.
    pub error: Option<ErrorAPI>,

    /// Possible delta body for [StreamEventType::ContentBlockDelta]
    /// responses.
    pub delta: Option<Delta>,

    /// Content block body.
    pub content_block: Option<Content>,

    /// Possible usage body, which is (usually) included in
    /// `message_delta` events.
    pub usage: Option<Usage>,

    /// Message body for message events;
    pub message: Option<MessageResponse>,

}

/// Enumerates the possible body types of a [Delta].
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeltaKind {
    TextDelta,
    InputJsonDelta,
    ThinkingDelta,
    SignatureDelta,
    ToolUse,
    Text,
    Thinking,
    ServerToolUse,
    WebSearchToolResult,
}

/// Body of a [ContentBlockDelta](StreamEventType::ContentBlockDelta)
/// streaming response.
#[derive(Debug, Serialize, Deserialize)]
pub struct Delta {

    /// The type of content block. None if the event is a
    /// `message_delta`, in which case `stop_reason` is populated.
    #[serde(rename = "type")]
    pub kind: Option<DeltaKind>,

    /// Text body for [DeltaKind::TextDelta] response types.
    pub text: Option<String>,

    /// Text body for [DeltaKind::InputJsonDelta] response types.
    /// This is used for `tool_use` content blocks.
    ///
    /// Current models only support emitting one complete key and
    /// value property from input at a time. As such, when using
    /// tools, there may be delays between streaming events while the
    /// model is working. Once an input key and value are accumulated,
    /// we emit them as multiple `content_block_delta` events with
    /// chunked partial json so that the format can automatically
    /// support finer granularity in future models.
    pub partial_json: Option<String>,

    /// Text body for [DeltaKind::ThinkingDelta] response types.
    pub thinking: Option<String>,

    /// Text body for [DeltaKind::SignatureDelta] response types.
    pub signature: Option<String>,

    /// The reason the content block stream ended.
    pub stop_reason: Option<StopReason>,

    /// Name of the tool being called.
    pub name: Option<String>,
}

/// Enumerates the possible content types of a [MessageResponse].
#[derive(Debug, Serialize, Deserialize)]
pub struct Content {

    /// The type of content response.
    #[serde(rename = "type")]
    pub kind: ContentKind,

    /// Text response data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Thinking response data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<String>,

    /// Thinking signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    /// Redacted thinking data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    /// Input to a tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,

    /// Name of a tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Tool use instance identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_use_id: Option<String>,
}

/// Message content type.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentKind {
    Text,
    Thinking,
    RedactedThinking,
    ToolUse,
    ServerToolUse,
    WebSearchToolUse,
    CodeExecutionToolResult,
    McpToolUse,
    McpToolResult,
    ContainerUpload,
}

/// Reports on the reason the LLM stopped generating tokens.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {

    /// The model reached a natural stopping point.
    EndTurn,

    /// Exceeded the requested max_tokens or the model's maximum.
    MaxTokens,

    /// One of the custom `stop_sequences` was generated.
    StopSequence,

    /// The model invoked one or more tools.
    ToolUse,

    /// Paused a long-running turn.
    PauseTurn,

    /// Potential policy violation.
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
#[serde(rename_all = "snake_case")]
pub enum ServiceTier {

    /// Default tier for both piloting and scaling everyday use cases.
    Standard,

    /// Best for workflows deployed in production where time,
    /// availability, and predictable pricing are important
    Priority,

    /// Best for asynchronous workflows.
    Batch,
}
