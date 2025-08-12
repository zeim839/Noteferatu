use serde::{Serialize, Deserialize};

/// OpenRouter API error response.
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub struct OpenRouterError {

    /// Message describing the error.
    pub message: String,

    /// Error code.
    pub code: i64,

    /// Provider metadata.
    pub metadata: Option<serde_json::Value>,
}

impl std::fmt::Display for OpenRouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
