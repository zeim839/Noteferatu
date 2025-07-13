use serde::{Serialize, Deserialize};

/// Chat message role.
#[derive(Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "system")]
    System,

    #[serde(rename = "developer")]
    Developer,

    #[serde(rename = "user")]
    User,

    #[serde(rename = "assistant")]
    Assistant,

    #[serde(rename = "tool")]
    Tool
}

/// Chat message.
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

/// ChatRequest represents a chat completion request.
///
/// [ChatRequest] must include either a [prompt](ChatRequest::prompt)
/// or [messages](ChatRequest::messages) field, but not both.
#[derive(Default, Serialize, Deserialize)]
pub struct ChatRequest {

    /// The model ID to use.
    pub model: String,

    /// The text prompt to complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// The message context to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,

    /// Configuration for model reasoning/thinking tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,

    /// Whether to include usage information in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageChoice>,

    /// Enable streaming of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Maximum number of tokens (range: [1, context_length)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,

    /// Sampling temperature (range: [0, 2]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<i64>,

    /// Seed for deterministic outputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    /// Top-p sampling value (range: (0, 1]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<i64>,

    /// Top-k sampling value (range: [1, Infinity)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i64>,

    /// Frequency penalty (range: [-2, 2]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<i64>,

    /// Presence penalty (range: [-2, 2]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<i64>,

    /// Repetition penalty (range: (0, 2]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<i64>,

    /// Number of top log probabilities to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i64>,

    /// Minimum probability threshold (range: [0, 1]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<i64>,

    /// Alternate top sampling parameter (range: [0, 1]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_a: Option<i64>,

}

impl ChatRequest {

    /// Creates a [ChatRequest] from a single text prompt.
    pub fn from_prompt(model: &str, prompt: &str) -> Self {
        let mut req = Self::default();
        req.model = model.to_string();
        req.prompt = Some(prompt.to_string());
        req
    }

    /// Creates a [ChatRequest] from a vector of messages.
    pub fn from_messages(model: &str, messages: Vec<Message>) -> Self {
        let mut req = Self::default();
        req.model = model.to_string();
        req.messages = Some(messages);
        req
    }

    /// Populates the `Reasoning` field with the given value.
    pub fn with_reasoning(self, reasoning: Option<Reasoning>) -> Self {
        Self { reasoning, ..self}
    }

    /// Populates the `usage` field with the given value.
    pub fn with_usage(self, usage: bool) -> Self {
        Self { usage: Some(UsageChoice { include: usage }), ..self }
    }

    /// Populates the `stream` field with the given value.
    pub fn with_stream(self, stream: Option<bool>) -> Self {
        Self { stream, ..self }
    }

    /// Populates the `max_tokens` field with the given value.
    pub fn with_max_tokens(self, max_tokens: Option<i64>) -> Self {
        Self { max_tokens, ..self }
    }

    /// Populates the `temperature` field with the given value.
    pub fn with_temperature(self, temperature: Option<i64>) -> Self {
        Self { temperature, ..self }
    }

    /// Populates the `seed` field with the given value.
    pub fn with_seed(self, seed: Option<i64>) -> Self {
        Self { seed, ..self }
    }

    /// Populates the `top_p` field with the given value.
    pub fn with_top_p(self, top_p: Option<i64>) -> Self {
        Self { top_p, ..self }
    }

    /// Populates the `top_k` field with the given value.
    pub fn with_top_k(self, top_k: Option<i64>) -> Self {
        Self { top_k, ..self }
    }

    /// Populates the `frequency_penalty` field with the given value.
    pub fn with_frequency_penalty(self, penalty: Option<i64>) -> Self {
        Self { frequency_penalty: penalty, ..self }
    }

    /// Populates the `presence_penalty` field with the given value.
    pub fn with_presence_penalty(self, penalty: Option<i64>) -> Self {
        Self { presence_penalty: penalty, ..self }
    }

    /// Populates the `repetition_penalty` field with the given value.
    pub fn with_repetition_penalty(self, penalty: Option<i64>) -> Self {
        Self { repetition_penalty: penalty, ..self }
    }

    /// Populates the `top_logprobs` field with the given value.
    pub fn with_top_logprobs(self, top_logprobs: Option<i64>) -> Self {
        Self { top_logprobs, ..self }
    }

    /// Populates the `min_p` field with the given value.
    pub fn with_min_p(self, min_p: Option<i64>) -> Self {
        Self { min_p, ..self }
    }

    /// Populates the `top_a` field with the given value.
    pub fn with_top_a(self, top_a: Option<i64>) -> Self {
        Self { top_a, ..self }
    }
}

/// Configuration for model reasoning/thinking tokens.
#[derive(Serialize, Deserialize)]
pub struct Reasoning {

    /// OpenAI-style reasoning effort setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<Effort>,

    /// Non-OpenAI-style reasoning effort setting. Cannot be used
    /// simultaneously with effort.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,

    /// Whether to exclude reasoning from the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<bool>,
}

/// OpenAI-style reasoning effort setting.
#[derive(Serialize, Deserialize)]
pub enum Effort {
    #[serde(rename = "high")]
    High,

    #[serde(rename = "medium")]
    Medium,

    #[serde(rename = "low")]
    Low,
}

/// Whether to include usage information in the response.
#[derive(Serialize, Deserialize)]
pub struct UsageChoice {

    /// Whether to include usage information in the response.
    pub include: bool,
}

/// ChatResponse represents a successful chat completion.
#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: Option<String>,
    pub choices: Option<Vec<Choice>>,
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub text: Option<String>,
    pub index: Option<i64>,
    pub finish_reason: Option<String>,
    pub message: Option<Message>,
}

/// Reports on the tokens consumed by a [ChatRequest] and its
/// subsequent [ChatResponse].
#[derive(Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}
