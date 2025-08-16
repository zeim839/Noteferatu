use serde::{Serialize, Deserialize};

/// OneDrive API error response.
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[serde(rename_all = "camelCase")]
pub struct OneDriveError {

    /// Message describing the error.
    pub message: String,

    /// Error code.
    pub code: String,
}

impl std::fmt::Display for OneDriveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
