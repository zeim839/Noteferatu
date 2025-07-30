use serde::{ser::Serializer, Serialize};
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, Serialize)]
#[serde(tag = "type", content = "error", rename_all = "camelCase")]
pub enum Error {
    #[error("{0}")]
    Io(String),

    #[error(transparent)]
    Helsync(#[from] crate::errors::Error),

    #[error("{0}")]
    Plugin(String),

    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

impl From<tauri::Error> for Error {
    fn from(error: tauri::Error) -> Self {
        Self::Plugin(error.to_string())
    }
}
