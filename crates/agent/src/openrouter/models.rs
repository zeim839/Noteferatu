use serde::{Serialize, Deserialize};

/// OpenRouter model metadata.
#[derive(Serialize, Deserialize)]
pub struct Model {

    /// Unique model identifier used in API requests (e.g.,
    /// "google/gemini-2.5-pro-preview")
    pub id: String,

    /// Human-readable display name for the model.
    pub name: String,

    /// Unix timestamp of when the model was added to OpenRouter.
    pub created: i64,

    /// Object describing the model’s technical capabilities.
    pub architecture: Architecture,

    /// Permanent slug for the model that never changes.
    pub canonical_slug: Option<String>,

    /// Maximum context window size in tokens.
    pub context_length: Option<i64>,

    /// Array of supported API parameters for this model.
    pub supported_parameters: Option<Vec<SupportedParams>>,
}

/// Object describing the model’s technical capabilities.
#[derive(Serialize, Deserialize)]
pub struct Architecture {

    /// Array of supported API parameters for this model.
    pub input_modalities: Vec<String>,

    /// Supported output types: ["text"].
    pub output_modalities: Vec<String>,

    /// Tokenization method used.
    pub tokenizer: String,

    /// Instruction format type (null if not applicable).
    pub instruct_type: Option<String>,
}

/// The SupportedParams enumerates which OpenAI-compatible parameters
/// work with each model.
#[derive(Serialize, Deserialize)]
pub enum SupportedParams {

    /// Function calling capabilities.
    #[serde(rename = "tools")]
    Tools,

    /// Tool selection control.
    #[serde(rename = "tool_choice")]
    ToolChoice,

    /// Response length limiting.
    #[serde(rename = "max_tokens")]
    MaxTokens,

    /// Randomness control.
    #[serde(rename = "temperature")]
    Temperature,

    /// Nucleus sampling.
    #[serde(rename = "top_p")]
    TopP,

    /// Top-k sampling value.
    #[serde(rename = "top_k")]
    TopK,

    /// Minimum probability threshold.
    #[serde(rename = "min_p")]
    MinP,

    /// Internal reasoning mode.
    #[serde(rename = "reasoning")]
    Reasoning,

    /// Include reasoning in response.
    #[serde(rename = "include_reasoning")]
    IncludeReasoning,

    /// JSON schema enforcement.
    #[serde(rename = "structured_outputs")]
    StructuredOutputs,

    /// Output format specification.
    #[serde(rename = "response_format")]
    ResponseFormat,

    /// Custom stop sequences.
    #[serde(rename = "stop")]
    Stop,

    /// Repetition reduction.
    #[serde(rename = "frequency_penalty")]
    FrequencyPenalty,

    /// Topic diversity.
    #[serde(rename = "presence_penalty")]
    PresencePenalty,

    /// Repetition penalty.
    #[serde(rename = "repetition_penalty")]
    RepetitionPenalty,

    /// Logit Bias.
    #[serde(rename = "logit_bias")]
    LogitBias,

    /// Log Probabilities.
    #[serde(rename = "logprobs")]
    LogProbs,

    /// Top Log Probabilities.
    #[serde(rename = "top_logprobs")]
    TopLogProbs,

    /// Web search options.
    #[serde(rename = "web_search_options")]
    WebSearchOptions,

    /// Top A.
    #[serde(rename = "top_a")]
    TopA,

    /// Deterministic outputs.
    #[serde(rename = "seed")]
    Seed,

}
