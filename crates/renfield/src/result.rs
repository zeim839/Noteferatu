use crate::client::Error as ClientError;

use crate::client::openai::Error as OpenAIError;
use crate::client::ollama::Error as OllamaError;
use crate::client::openrouter::Error as OpenRouterError;
use crate::client::gemini::Error as GeminiError;
use crate::client::anthropic::Error as AnthropicError;

/// Result alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Error implementation.
#[derive(Debug, thiserror::Error)]
pub enum Error {

    /// An HTTP client error.
    #[error("{0}")]
    Http(#[from] reqwest::Error),

    /// LLM client error.
    #[error("{0}")]
    Client(#[from] ClientError),

    /// A JSON decoding error.
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

// Directly convert API error responses to the [Error] type.
// Convenient for client implementation (saves us the intermediate
// step of having to convert to [crate::client::Error]).

impl From<OpenAIError> for Error {
    fn from(value: OpenAIError) -> Self {
        crate::client::Error::from(value).into()
    }
}

impl From<OllamaError> for Error {
    fn from(value: OllamaError) -> Self {
        crate::client::Error::from(value).into()
    }
}

impl From<OpenRouterError> for Error {
    fn from(value: OpenRouterError) -> Self {
        crate::client::Error::from(value).into()
    }
}

impl From<GeminiError> for Error {
    fn from(value: GeminiError) -> Self {
        crate::client::Error::from(value).into()
    }
}

impl From<AnthropicError> for Error {
    fn from(value: AnthropicError) -> Self {
        crate::client::Error::from(value).into()
    }
}
