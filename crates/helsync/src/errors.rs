use std::fmt::{Display, Formatter};
use std::time::SystemTimeError;

/// [Helsync](crate) result alias.
pub type Result<T> = std::result::Result<T, Error>;

/// [Helsync](crate) error type.
#[derive(Debug)]
pub enum Error {
    Client(reqwest::Error),
    Json(serde_json::Error),
    SystemTime(SystemTimeError),
    Database(sqlx::Error),
    Io(std::io::Error),
    Other(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Client(err) => write!(f, "client error: {err}"),
            Error::Json(err) => write!(f, "json error: {err}"),
            Error::SystemTime(err) => write!(f, "system time error: {err}"),
            Error::Database(err) => write!(f, "database error: {err}"),
            Error::Other(err) => write!(f, "{err}"),
            Error::Io(err) => write!(f, "io error: {err}"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Client(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<SystemTimeError> for Error {
    fn from(error: SystemTimeError) -> Self {
        Self::SystemTime(error)
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl std::error::Error for Error {}
