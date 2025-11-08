use serde::{Serialize, Deserialize};

/// Gemini API error.
#[derive(thiserror::Error, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Error {

    /// Error code (same as HTTP status).
    pub code: i64,

    /// Message describing the error.
    pub message: String,

    /// Error status.
    pub status: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
