use serde::{Serialize, Deserialize};

/// Ollama API error response object.
///
/// See: [API Reference](https://docs.ollama.com/api/errors)
#[derive(Serialize, Deserialize, Debug, thiserror::Error, Default)]
#[error("{error}")]
pub struct Error {
    pub error: String,
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Self { error }
    }
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Self { error: error.to_string() }
    }
}
