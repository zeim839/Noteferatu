use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, Serialize)]
#[serde(tag = "type", content = "error", rename_all = "camelCase")]
pub enum Error {
    #[error("io: {0}")]
    Io(String),

    #[error(transparent)]
    Api(#[from] agent::Error),

    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}
