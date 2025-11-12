use serde::{Serialize, Deserialize};

/// Anthropic API error response.
#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Error {

    /// Message describing the error.
    pub message: String,

    /// The type of error.
    #[serde(rename = "type")]
    pub kind: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
