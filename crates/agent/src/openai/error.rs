use serde::{Serialize, Deserialize};

/// An OpenAI error.
///
/// API Reference: [Error Codes](https://platform.openai.com/docs/guides/error-codes)
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorAPI {
    pub message: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub code: String,
}

impl std::fmt::Display for ErrorAPI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) {}", self.code, self.message)
    }
}
