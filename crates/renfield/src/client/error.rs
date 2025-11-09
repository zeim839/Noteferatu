use serde::Serialize;

use super::openai::Error as OpenAIError;
use super::ollama::Error as OllamaError;
use super::openrouter::Error as OpenRouterError;
use super::gemini::Error as GeminiError;
use super::anthropic::Error as AnthropicError;

/// Error enumerates the API error responses of different LLM clients.
///
/// Clients may also fail without an error response from an API
/// endpoint (e.g. if the HTTP connection times out, or if JSON
/// decoding fails). Hence, clients use the broader [crate::Error]
/// type, which encapsulates API error responses as well.
///
/// Custom [Client](super::Client) implementations need not use this
/// error type. The [Client](super::Client) trait supports specifying
/// custom error types.
#[derive(thiserror::Error, Debug, Serialize)]
#[serde(tag = "provider", rename_all = "lowercase")]
pub enum Error {

    /// An [OpenAI](super::openai) error response object.
    #[error("{0}")]
    OpenAI(#[from] OpenAIError),

    /// An [Ollama](super::ollama) error response object.
    #[error("{0}")]
    Ollama(#[from] OllamaError),

    /// An [OpenRouter](super::openrouter) error response object.
    #[error("{0}")]
    OpenRouter(#[from] OpenRouterError),

    /// A [Gemini](super::gemini) error response object.
    #[error("{0}")]
    Gemini(#[from] GeminiError),

    /// An [Anthropic](super::anthropic) error response object.
    #[error("{0}")]
    Anthropic(#[from] AnthropicError),
}
