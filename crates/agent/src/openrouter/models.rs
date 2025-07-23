use serde::{Serialize, Deserialize};
use crate::client::GenerationParam;

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
    pub supported_parameters: Option<Vec<GenerationParam>>,
}

/// Object describing the model’s technical capabilities.
#[derive(Debug, Serialize, Deserialize)]
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

impl Into<crate::Model> for Model {
    fn into(self) -> crate::Model {
        crate::Model {
            id: self.id,
            display_name: self.name,
            provider: "OpenRouter".to_string(),
            context_size: self.context_length.unwrap_or(20000) as u64,
        }
    }
}
