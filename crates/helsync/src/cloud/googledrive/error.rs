use serde::{Serialize, Deserialize};

/// Google Drive API error response.
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[serde(rename_all = "camelCase")]
pub struct GoogleDriveError {

    /// Error code.
    pub code: i64,

    /// Message describing the error.
    pub message: String,
}

impl std::fmt::Display for GoogleDriveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
