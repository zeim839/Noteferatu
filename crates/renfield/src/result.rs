use crate::client::Error as ClientError;

use crate::client::openai::Error as OpenAIError;

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
