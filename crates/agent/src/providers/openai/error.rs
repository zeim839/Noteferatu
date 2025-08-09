use serde::{Serialize, Deserialize};

/// OpenAI API error response.
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[serde(rename_all = "camelCase")]
pub struct OpenAIError {

    /// Message describing the error.
    pub message: String,

    /// The type of error.
    #[serde(rename = "type")]
    pub kind: String,

    /// Error code.
    pub code: Option<String>,
}

impl std::fmt::Display for OpenAIError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
