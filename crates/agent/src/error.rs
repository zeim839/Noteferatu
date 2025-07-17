use std::fmt::{Display, Formatter, Result};

/// A generic LLM client error.
#[derive(Debug)]
pub enum Error<T: core::fmt::Debug> {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Stream(String),
    Api(T),
}

impl<T: Display + core::fmt::Debug> Display for Error<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::Http(e) => write!(f, "HTTP error: {e}"),
            Error::Json(e) => write!(f, "JSON error: {e}"),
            Error::Api(e) => write!(f, "API error: {e}"),
            Error::Stream(e) => write!(f, "Stream error: {e}"),
        }
     }
}

impl<T: Display + core::fmt::Debug> From<reqwest::Error> for Error<T> {
    fn from(error: reqwest::Error) -> Self {
        Error::Http(error)
    }
}

impl<T: Display + core::fmt::Debug> From<serde_json::Error> for Error<T> {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

impl<T: Display + core::fmt::Debug> std::error::Error for Error<T> {}
