use serde::{Serialize, Deserialize};

/// Common interface for LLM API clients.
pub trait Client {

    /// The client-specific chat completion request type.
    type Request: Into<Request>;

    /// A non-streaming response to a chat completion request.
    type StaticResponse: Into<Response>;

    /// A streaming response to a chat completion request.
    type StreamResponse: std::iter::Iterator<Item: Into<Response>>;

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

pub struct Request {
}

pub struct Response {
}
