//! Model information.

use serde::Deserialize;

/// A model available on OpenRouter.
///
/// See: [API Reference](https://openrouter.ai/docs/api-reference/models/get-models#response.body.data)
#[derive(Deserialize, Debug, Clone)]
pub struct Model {

    /// Unique identifier for the model.
    pub id: String,

    /// Canonical slug for the model.
    pub canonical_slug: String,

    /// Display name of the model.
    pub name: String,

    /// Unix timestamp of when the model was created.
    pub created: u64,

    /// Pricing information for the model.
    pub pricing: Pricing,

    /// Maximum context length in tokens.
    pub context_length: Option<u64>,

    /// Model architecture information
    pub architecture: ModelArchitecture,

    /// Information about the top provider for this model.
    pub top_provider: TopProvider,

    /// Per-request token limits.
    pub per_request_limits: Option<PerRequestLimits>,

    /// List of supported parameters for this model.
    pub supported_parameters: Vec<String>,

    /// Hugging Face model identifier, if applicable.
    pub hugging_face_id: Option<String>,

    /// Description of the model.
    pub description: Option<String>,
}

/// Pricing information for the model.
///
/// See: [API Reference](https://openrouter.ai/docs/api-reference/models/get-models#response.body.data.pricing)
#[derive(Deserialize, Debug, Clone)]
pub struct Pricing {

    /// Price per prompt.
    pub prompt: Price,

    /// Price per completion.
    pub completion: Price,

    /// Price per request.
    pub request: Option<Price>,

    /// Price per input image.
    pub image: Option<Price>,

    /// Price per output image.
    pub image_output: Option<Price>,

    /// Price per audio input.
    pub audio: Option<Price>,

    /// Price for input audio cache usage.
    pub input_audio_cache: Option<Price>,

    /// Price per web search result.
    pub web_search: Option<Price>,

    /// Price for internal reasoning traces.
    pub internal_reasoning: Option<Price>,

    /// Price per input cache read.
    pub input_cache_read: Option<Price>,

    /// Price per input cache write.
    pub input_cache_write: Option<Price>,

    /// Discounts, if available.
    pub discount: Option<f64>,
}

/// Model [Price](Pricing) that is either a String or Double.
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Price {
    String(String),
    Double(f64),
}

/// Model architecture information.
#[derive(Deserialize, Debug, Clone)]
pub struct ModelArchitecture {

    /// Primary modality of the model.
    pub modality: Option<String>,

    /// Supported input modalities.
    pub input_modalities: Vec<Modality>,

    /// Supported output modalities.
    pub output_modalities: Vec<Modality>,

    /// Tokenizer type used by the model.
    pub tokenizer: Option<String>,

    /// Instruction format type.
    pub instruct_type: Option<String>,
}

/// Model input/output modalities.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Modality {
    Text,
    Image,
    File,
    Audio,
    Video,
    Embeddings,
}

/// Information about the top provider for a model.
#[derive(Deserialize, Debug, Clone)]
pub struct TopProvider {

    /// Whether the top provider moderates content.
    pub is_moderated: bool,

    /// Context length from the top provider.
    pub context_length: Option<u64>,

    /// Maximum completion tokens from the top provider.
    pub max_completion_tokens: Option<u64>,
}

/// Per-request token limits.
#[derive(Deserialize, Debug, Clone)]
pub struct PerRequestLimits {

    /// Maximum prompt tokens per request.
    pub prompt_tokens: u64,

    /// Maximum completion tokens per request.
    pub completion_tokens: u64,
}

/// Default output/sampling parameters for this model.
#[derive(Deserialize, Debug, Clone)]
pub struct DefaultParameters {

    /// Sampling temperature.
    pub temperature: Option<f64>,

    /// Top_p sampling parameter.
    pub top_p: Option<f64>,

    /// Frequency penalty.
    pub frequency_penalty: Option<f64>,
}
