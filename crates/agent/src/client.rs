use serde::{Serialize, Deserialize};
use crate::openai::FunctionDefinition;

/// Common interface for LLM API clients.
pub trait Client {

    /// The client-specific chat completion request type.
    type Request: Request;

    /// A non-streaming response to a chat completion request.
    type StaticResponse: Response;

    /// A streaming response to a chat completion request.
    type StreamResponse: std::iter::Iterator<Item: Response>;

    /// Definition & capabilities of an LLM.
    type ModelDefinition: Into<Model>;

    /// The client-specific error implementation.
    type Error: std::error::Error;

    /// Generate a non-streaming chat completion.
    fn completion(&self, model: &str, req: &Self::Request) ->
        Result<Self::StaticResponse, Self::Error>;

    /// Generate a streaming chat completion.
    fn stream_completion(&self, model: &str, req: &Self::Request) ->
        Result<Self::StreamResponse, Self::Error>;

    /// List the models available on the client.
    fn list_models(&self) ->
        Result<Vec<Self::ModelDefinition>, Self::Error>;
}

/// Implements a generic LLM definition that captures basic attributes
/// from all clients.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub id: String,
    pub display_name: String,
    pub provider: String,
    pub context_size: u64,
}

/// Completion generation parameters.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationParam {

    /// Function calling capabilities.
    Tools,

    /// Tool selection control.
    ToolChoice,

    /// Call multiple tools within a single response.
    ParallelToolCalls,

    /// Response length limiting.
    MaxTokens,

    /// Randomness control.
    Temperature,

    /// Nucleus sampling.
    TopP,

    /// Top-k sampling value.
    TopK,

    /// Minimum probability threshold.
    MinP,

    /// Include reasoning configuration.
    Reasoning,

    /// Include reasoning in response.
    IncludeReasoning,

    /// JSON schema enforcement.
    StructuredOutputs,

    /// Output format specification.
    ResponseFormat,

    /// Custom stop sequences.
    Stop,

    /// Repetition reduction.
    FrequencyPenalty,

    /// Topic diversity.
    PresencePenalty,

    /// Repetition penalty.
    RepetitionPenalty,

    /// Logit Bias.
    LogitBias,

    /// Log Probabilities.
    Logprobs,

    /// Top Log Probabilities.
    TopLogprobs,

    /// Web search options.
    WebSearchOptions,

    /// Top A.
    TopA,

    /// Deterministic outputs.
    Seed,
}

/// Common interface for chat completion requests.
pub trait Request {

    /// Set the maximum chat completion output tokens.
    fn with_max_tokens(self, max_tokens: Option<i64>) -> Self;

    /// Set the model temperature parameter.
    fn with_temperature(self, temperature: Option<f64>) -> Self;

    /// Configure web search results options.
    fn with_web_search_results(self, web_search_results: Option<i64>) -> Self;

    /// Configure tool calling.
    fn with_tools(self, tools: Option<Vec<FunctionDefinition>>) -> Self;
}

/// Common interface for chat completion responses.
pub trait Response {

    /// Get the text response, if available.
    fn get_text(&self) -> Option<String>;

    /// Get the tool call ID, if available.
    fn get_tool_call_id(&self) -> Option<String>;

    /// Get tool calls, if available.
    fn get_tool_calls(&self) -> Option<Vec<()>>;

    /// Get refusal, if available.
    fn get_refusal(&self) -> Option<String>;

    /// Get thinking traces, if available.
    fn get_thinking(&self) -> Option<String>;

    /// Get usage data, if available.
    fn get_usage(&self) -> Option<()>;
}
