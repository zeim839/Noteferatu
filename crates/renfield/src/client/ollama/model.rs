use serde::Deserialize;
use std::collections::HashMap;

/// Ollama model specification.
#[derive(Deserialize, Debug, Clone)]
pub struct Model {

    /// Model name.
    pub name: String,

    /// Last modified timestamp in ISO 8601 format.
    pub modified_at: String,

    /// Total size of the model on disk in bytes.
    pub size: u64,

    /// SHA256 digest identifier of the model contents.
    pub digest: String,

    /// Additional information about the model's format and family.
    pub details: ModelDetails,
}

/// Additional information about the model.
#[derive(Deserialize, Debug, Clone)]
pub struct ModelDetails {

    /// Model file format.
    ///
    /// For example, `gguf`.
    pub format: String,

    /// Primary model family
    ///
    /// For example, `llama`.
    pub family: String,

    /// All families the model belongs to, when applicable.
    pub families: Vec<String>,

    /// Approximate parameter count label (for example `7B`, `13B`).
    pub parameter_size: String,

    /// Quantization level used.
    ///
    /// For example, `Q4_0`.
    pub quantization_level: String,
}

/// Even more model details.
///
/// This is the metadata object returned by the Ollama [Show Model
/// Details](https://docs.ollama.com/api-reference/show-model-details)
/// route.
///
/// There is no real consistency in [ModelMetadata]. Field values vary
/// from model to model. I wish the Ollama API was better...
#[derive(Deserialize, Debug, Clone)]
pub struct ModelMetadata {

    /// Model parameter settings serialized as text.
    pub parameters: String,

    /// The license of the model.
    pub license: String,

    /// High-level model details.
    pub details: HashMap<String, serde_json::Value>,

    /// The template used by the model to render prompts.
    pub template: String,

    /// List of supported features.
    pub capabilities: Vec<String>,

    /// Additional model metadata.
    pub model_info: HashMap<String, serde_json::Value>,
}
