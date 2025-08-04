use serde::Serialize;

use crate::providers::anthropic::AnthropicError;
use crate::providers::google::GoogleError;
use crate::providers::openai::OpenAIError;

/// Agent result alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Agent error implementation.
#[derive(thiserror::Error, Debug, Serialize)]
pub enum Error {

    /// An HTTP client error.
    #[error("web client: {0}")]
    Client(String),

    /// Anthropic API error.
    #[error("{0}")]
    Anthropic(#[from] AnthropicError),

    /// Google API error.
    #[error("{0}")]
    Google(#[from] GoogleError),

    /// Ollama API error.
    #[error("{0}")]
    Ollama(String),

    /// OpenAI API error.
    #[error("{0}")]
    OpenAI(#[from] OpenAIError),

    /// OpenRouter API error.
    #[error("{0}")]
    OpenRouter(OpenAIError),

    /// A JSON decoding error.
    #[error("json: {0}")]
    Json(String),

    /// An invalid model ID was provided.
    #[error("invalid model identifier: {0}")]
    InvalidModelId(String),

    /// The requested provider is not configured.
    #[error("provider not configured: {0}")]
    ProviderNotConfigured(String),

    /// An SQL [database] error.
    #[error("database: {0}")]
    Sql(String),

    #[cfg(feature = "plugin")]
    #[error("io: {0}")]
    Io(String),

    #[cfg(feature = "plugin")]
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Client(error.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Sql(error.to_string())
    }
}
