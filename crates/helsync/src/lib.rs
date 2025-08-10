//! ![Abraham Van Helsing](https://upload.wikimedia.org/wikipedia/commons/3/38/Dracula1931BelaLugosiColorCrop.jpg)

pub(crate) mod client;
pub(crate) mod utils;

mod filesystem;
pub use filesystem::*;

mod errors;
pub use errors::*;

pub mod googledrive;
pub mod oauth2;
pub mod onedrive;
pub mod local;
pub mod sync;

#[cfg(feature = "plugin")]
pub mod plugin;
