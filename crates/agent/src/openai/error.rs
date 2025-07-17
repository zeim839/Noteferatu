use serde::{Serialize, Deserialize};
use crate::error::Error as E;

/// An OpenAI error.
///
/// API Reference: [Error Codes](https://platform.openai.com/docs/guides/error-codes)
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIError {
    pub message: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub code: String,
}

impl std::fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

/// Alias for the OpenAI error type.
pub type Error = E<OpenAIError>;
