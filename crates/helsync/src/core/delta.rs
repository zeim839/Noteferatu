use crate::core::Result;
use super::file::File;

use std::future::Future;

/// Defines synchronization primitives across
/// [FileSystem](super::filesystem::FileSystem)'s.
pub trait Delta {
    type File: Into<File>;

    /// Fetches the latest state of the filesystem.
    ///
    /// `token` is an optional token for omitting processed
    /// changes.
    fn list_deltas(&self, token: Option<&str>) ->
    impl Future<Output = Result<(Vec<Self::File>, String)>>;
}
