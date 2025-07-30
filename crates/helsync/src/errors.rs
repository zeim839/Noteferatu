use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter};
use std::time::SystemTimeError;

/// [Helsync](crate) result alias.
pub type Result<T> = std::result::Result<T, Error>;

/// [Helsync](crate) error type.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "error", rename_all = "camelCase")]
pub enum Error {
    Client(String),
    Json(String),
    SystemTime(String),
    Database(String),
    Io(String),
    Other(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Client(err) => write!(f, "{err}"),
            Error::Json(err) => write!(f, "{err}"),
            Error::SystemTime(err) => write!(f, "{err}"),
            Error::Database(err) => write!(f, "{err}"),
            Error::Other(err) => write!(f, "{err}"),
            Error::Io(err) => write!(f, "{err}"),
        }
    }
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

impl From<SystemTimeError> for Error {
    fn from(error: SystemTimeError) -> Self {
        Self::SystemTime(error.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

impl std::error::Error for Error {}
