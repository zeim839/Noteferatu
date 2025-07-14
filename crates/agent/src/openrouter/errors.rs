use serde::{Serialize, Deserialize};

/// Error returned from [Client](super::Client).
#[derive(Debug)]
pub enum ClientError {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Api(OpenRouterError),
    Stream(String),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ClientError::Http(e) => write!(f, "HTTP error: {}", e),
            ClientError::Json(e) => write!(f, "JSON error: {}", e),
            ClientError::Api(e) => write!(f, "API error: {} (code: {})", e.message, e.code),
            ClientError::Stream(e) => write!(f, "Stream error: {}", e),
        }
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        ClientError::Http(error)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(error: serde_json::Error) -> Self {
        ClientError::Json(error)
    }
}

impl std::error::Error for ClientError {}

/// An OpenRouter error.
///
/// See [Error Reference](https://openrouter.ai/docs/api-reference/errors)
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenRouterError {
    pub code: i64,
    pub message: String,
    pub metadata: Option<ErrorMetadata>,
}

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
