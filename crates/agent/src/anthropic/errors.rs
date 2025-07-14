use serde::{Serialize, Deserialize};

/// Error returned from [Client](super::Client).
#[derive(Debug)]
pub enum ClientError {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Api(AnthropicError),
    Stream(String),
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ClientError::Http(e) => write!(f, "HTTP error: {}", e),
            ClientError::Json(e) => write!(f, "JSON error: {}", e),
            ClientError::Api(e) => write!(f, "API error: {} (type: {})", e.message, e.kind),
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

/// An Anthropic API error.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicError {
    pub message: String,
    #[serde(rename = "type")]
    pub kind: String,
}
