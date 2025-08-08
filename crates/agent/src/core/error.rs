use serde::Serialize;

#[cfg(feature = "anthropic")]
use crate::providers::anthropic::AnthropicError;

#[cfg(feature = "google")]
use crate::providers::google::GoogleError;

#[cfg(feature = "openai")]
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
    OpenRouter(OpenAIError),

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
