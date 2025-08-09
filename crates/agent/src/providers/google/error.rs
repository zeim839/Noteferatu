use serde::{Serialize, Deserialize};

/// Google API error response.
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[serde(rename_all = "camelCase")]
pub struct GoogleError {

    /// Error code (same as HTTP status).
    pub code: i64,

    /// Message describing the error.
    pub message: String,

    /// Error status.
    pub status: String,
}

impl std::fmt::Display for GoogleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
