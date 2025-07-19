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

impl crate::ModelDefinition for Model {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn display_name(&self) -> String {
        self.name.clone()
    }

    fn context_length(&self) -> u64 {
        self.context_length.unwrap_or(20000) as u64
    }

    fn supports_tool_calls(&self) -> bool {
        use crate::GenerationParam::Tools;
        self.supported_parameters.as_ref()
            .is_some_and(|params| params.iter().find(|p| **p == Tools).is_some())
    }

    fn supports_web_search(&self) -> bool {
        true
    }
}
