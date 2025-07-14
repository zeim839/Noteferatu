use serde::{Serialize, Deserialize};

/// Error returned from [Client](super::Client).
#[derive(Debug)]
pub enum ClientError {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Api(OpenAIError),
    Stream(String),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ClientError::Http(e) => write!(f, "HTTP error: {}", e),
            ClientError::Json(e) => write!(f, "JSON error: {}", e),
            ClientError::Api(e) => write!(f, "API error: {} (code: {})", e.message, e.code),
            ClientError::Stream(e) => write!(f, "Stream error: {}", e),
        }
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        ClientError::Http(error)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(error: serde_json::Error) -> Self {
        ClientError::Json(error)
    }
}

impl std::error::Error for ClientError {}

/// An OpenAI error.
///
/// API Reference: [Error Codes](https://platform.openai.com/docs/guides/error-codes)
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIError {
    pub message: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub code: String,
}
