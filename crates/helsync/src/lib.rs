//! ![Abraham Van Helsing](https://upload.wikimedia.org/wikipedia/commons/3/38/Dracula1931BelaLugosiColorCrop.jpg)

pub mod cloud;
pub mod core;
pub mod oauth2;
pub mod local;
pub mod sync;

#[cfg(feature = "plugin")]
pub mod plugin;
