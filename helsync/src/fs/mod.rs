//! Local & Remote Virtual Filesystems

mod fs;
pub use fs::*;

pub mod onedrive;
pub mod googledrive;
pub mod local;

mod client;
mod utils;
