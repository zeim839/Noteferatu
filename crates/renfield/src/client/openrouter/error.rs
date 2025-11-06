use serde::{Serialize, Deserialize};

/// OpenRouter API error.
///
/// See: [API Reference](https://openrouter.ai/docs/api-reference/errors)
#[derive(thiserror::Error, Debug, Serialize, Deserialize, Default)]
pub struct Error {

    /// Error code.
    pub code: i64,

    /// Message describing the error.
    pub message: String,

    /// Provider metadata.
    pub metadata: Option<serde_json::Value>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
