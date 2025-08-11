use serde::{Serialize, Deserialize};

/// Describes a model available via the API.
///
/// API Reference: [Models](https://openrouter.ai/docs/api-reference/list-available-models)
#[derive(Debug, Serialize, Deserialize)]
pub struct Model {

    /// Unique model identifier.
    pub id: String,

    /// Name of the model.
    pub name: String,

    /// Model creation timestamp (unix).
    pub created: i64,

    /// Model description.
    pub description: String,

    /// Describes the input/output modalities and architecture of the
    /// model.
    pub architecture: ModelArchitecture,

    /// The model's top provider on OpenRouter.
    ///
    /// OpenRouter models are hosted by providers, which set the price
    /// and features available for each model. The `top_provider`
    /// attribute specifies the top provider for this model.
    pub top_provider: ModelProvider,

    /// Model pricing data.
    pub pricing: ModelPricing,

    /// Canonical slug identifier for the model.
    pub canonical_slug: Option<String>,

    /// Context size for the model.
    pub context_length: Option<u64>,

    /// Hugging face ID (available for some open weight models).
    pub hugging_face_id: Option<String>,

    /// The chat completion API request parameters supported by the
    /// model.
    #[serde(default)]
    pub supported_parameters: Vec<String>,
}

impl Into<crate::core::Model> for Model {
    fn into(self) -> crate::core::Model {
        crate::core::Model {
            id: self.id.clone(),
            display_name: self.id.clone(),
            provider: "OpenRouter".to_string(),
            context_size: self.context_length.unwrap_or(200000),
        }
    }
}

/// Describes an OpenRouter model's architecture.
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelArchitecture {

    /// The input modalities supported by the model.
    pub input_modalities: Vec<String>,

    /// The output modalities supported by the model.
    pub output_modalities: Vec<String>,

    /// The tokenizer used to convert plain text to tokens.
    pub tokenizer: String,

    /// The model's instruct type.
    pub instruct_type: Option<String>,
}

/// OpenRouter model pricing data.
///
/// Prices are per token, or per result for web search, cache reads,
/// etc. They are encoded as strings to support high precision (small)
/// values.
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelPricing {

    /// Price per prompt token.
    pub prompt: Option<String>,

    /// Price per completion token.
    pub completion: Option<String>,

    /// Price per image read (OpenRouter does not support generation).
    pub image: Option<String>,

    /// Price per request.
    pub request: Option<String>,

    /// Price per web search result.
    ///
    /// The default is 5 results per web search request.
    pub web_search: Option<String>,

    /// Internal reasoning per-token price.
    pub internal_reasoning: Option<String>,

    /// Cache read price.
    pub input_cache_read: Option<String>,

    /// Cache write price
    pub input_cache_write: Option<String>,
}

/// OpenRouter provider information.
///
/// OpenRouter models are hosted by providers, which set the price
/// and features available for each model.
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelProvider {

    /// Whether the provider moderates LLM outputs.
    pub is_moderated: bool,

    /// The context-length offered or a model by this provider.
    pub context_length: Option<u64>,

    /// The maximum allowable completion tokens by this provider.
    pub max_completion_tokens: Option<u64>,
}
