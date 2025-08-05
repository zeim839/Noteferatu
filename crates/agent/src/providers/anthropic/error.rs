use serde::{Serialize, Deserialize};

/// Anthropic API error response.
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub struct AnthropicError {

    /// Message describing the error.
    pub message: String,

    /// The type of error.
    #[serde(rename = "type")]
    pub kind: String,
}

impl std::fmt::Display for AnthropicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
