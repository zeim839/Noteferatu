use serde::{Serialize, Deserialize};
use crate::error::Error as E;

/// An OpenRouter error.
///
/// See [Error Reference](https://openrouter.ai/docs/api-reference/errors)
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenRouterError {
    pub code: i64,
    pub message: String,
    pub metadata: Option<ErrorMetadata>,
}

impl std::fmt::Display for OpenRouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

/// Alias for the OpenRouter error.
pub type Error = E<OpenRouterError>;

/// Enumerates the possible OpenRouter error metadata object bodies.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorMetadata {
    Moderation(ModerationError),
    Provider(ProviderError),
}

/// If the input was flagged for moderation errors, the
/// ModerationError metadata will contain information about the issue.
#[derive(Debug, Serialize, Deserialize)]
pub struct ModerationError {

    /// Why your input was flagged.
    reasons: Vec<String>,

    /// The text segment that was flagged, limited to 100
    /// characters. If the flagged input is longer than 100
    /// characters, it will be truncated in the middle and replaced
    /// with ...
    flagged_input: String,

    /// The name of the provider that requested moderation.
    provider_name: String,

    /// References the associated OpenRouter
    /// [model_slug](super::Model::canonical_slug).
    model_slug: String,
}

/// If the model provider encounters an error, the ProviderError
/// metadata will contain information about the issue.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderError {

    /// The name of the provider that encountered the error
    provider_name: String,

    /// The raw error from the provider
    raw: Option<serde_json::Value>,
}
