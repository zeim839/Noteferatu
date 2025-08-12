use serde::{Serialize, Deserialize};

#[cfg(feature = "anthropic")]
use crate::providers::anthropic::AnthropicError;

#[cfg(feature = "google")]
use crate::providers::google::GoogleError;

#[cfg(feature = "openai")]
use crate::providers::openai::OpenAIError;

#[cfg(feature = "openrouter")]
use crate::providers::openrouter::OpenRouterError;

/// Agent result alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Agent error implementation.
#[derive(thiserror::Error, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "type", content = "data")]
pub enum Error {

    /// An HTTP client error.
    #[error("{0}")]
    Client(#[from] ClientError),

    /// Anthropic API error.
    #[cfg(feature = "anthropic")]
    #[error("{0}")]
    Anthropic(#[from] AnthropicError),

    /// Google API error.
    #[cfg(feature = "google")]
    #[error("{0}")]
    Google(#[from] GoogleError),

    /// Ollama API error.
    #[cfg(feature = "ollama")]
    #[error("{0}")]
    Ollama(String),

    /// OpenAI API error.
    #[cfg(feature = "openai")]
    #[error("{0}")]
    OpenAI(#[from] OpenAIError),

    /// OpenRouter API error.
    #[cfg(feature = "openrouter")]
    #[error("{0}")]
    OpenRouter(OpenRouterError),

    /// A JSON decoding error.
    #[error("json: {0}")]
    Json(String),

    /// An invalid model ID was provided.
    #[cfg(feature = "agent")]
    #[error("invalid model identifier: {0}")]
    InvalidModelId(String),

    /// The requested provider is not configured.
    #[cfg(feature = "agent")]
    #[error("provider not configured: {0}")]
    ProviderNotConfigured(String),

    /// An SQL [database] error.
    #[cfg(feature = "agent")]
    #[error("database: {0}")]
    Sql(String),

    #[cfg(feature = "plugin")]
    #[error("io: {0}")]
    Io(String),

    #[cfg(feature = "plugin")]
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),

    #[cfg(feature = "plugin")]
    #[error("{0}")]
    Plugin(String),
}

/// Serializable [reqwest] error type.
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[serde(rename_all = "camelCase")]
pub struct ClientError {

    /// HTTP status for when the error is from an HTTP error response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,

    /// Error message.
    pub message: String,

    /// A possible URL related to this error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        if error.is_redirect() {
            Self::Client(ClientError{
                status: None,
                message: "redirect policy error".to_string(),
                url: error.url().map(|url| url.as_str().to_string()),
            })
        }
        else if error.is_status() {
            Self::Client(ClientError{
                status: error.status().map(|status| status.as_u16()),
                message: "received error status response".to_string(),
                url: error.url().map(|url| url.as_str().to_string()),
            })
        }
        else if error.is_timeout() {
            Self::Client(ClientError{
                status: None,
                message: "request timed out".to_string(),
                url: error.url().map(|url| url.as_str().to_string()),
            })
        }
        else if error.is_request() {
            Self::Client(ClientError{
                status: None,
                message: "bad request".to_string(),
                url: error.url().map(|url| url.as_str().to_string()),
            })
        }
        else if error.is_connect() {
            Self::Client(ClientError{
                status: None,
                message: "connection error".to_string(),
                url: error.url().map(|url| url.as_str().to_string()),
            })
        }
        else if error.is_body() {
            Self::Client(ClientError{
                status: None,
                message: "invalid request or response body".to_string(),
                url: error.url().map(|url| url.as_str().to_string()),
            })
        }
        else if error.is_decode() {
            Self::Client(ClientError{
                status: None,
                message: "could not decode response body".to_string(),
                url: error.url().map(|url| url.as_str().to_string()),
            })
        }
        else {
            Self::Client(ClientError{
                status: None,
                message: error.to_string(),
                url: error.url().map(|url| url.as_str().to_string()),
            })
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error.to_string())
    }
}

#[cfg(feature = "agent")]
impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Sql(error.to_string())
    }
}

#[cfg(feature = "plugin")]
impl From<tauri::Error> for Error {
    fn from(error: tauri::Error) -> Self {
        Self::Plugin(error.to_string())
    }
}
