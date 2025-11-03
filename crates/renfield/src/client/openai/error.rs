use serde::{Serialize, Deserialize};

/// OpenAI API error.
#[derive(thiserror::Error, Debug, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(rename = "type")]
    pub kind: String,
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
