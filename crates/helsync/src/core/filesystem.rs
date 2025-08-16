use std::future::Future;
use super::file::File;

/// Defines a file system interface.
pub trait FileSystem {
    type File: Into<File>;
    type Error: std::error::Error;

    /// Fetches metadata for the file with `id`.
    fn get_file(&self, id: &str) ->
    impl Future<Output = Result<Self::File, Self::Error>>;

    /// Copy a file.
    ///
    /// Copies a file and moves it under `parent_id`. If `parent_id` is
    /// unspecified, then the file is moved to the root directory. If
    /// `name` is specified, the file is renamed.
    fn copy_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) ->
    impl Future<Output = Result<Self::File, Self::Error>>;

    /// Move a file.
    ///
    /// Sets the file's parent to `parent_id`. If `parent_id` is
    /// unspecified, then the file is moved to the root directory. If
    /// `name` is specified, the file is renamed.
    fn move_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) ->
    impl Future<Output = Result<Self::File, Self::Error>>;

    /// Delete the file with the given `id`.
    fn remove_file(&self, id: &str) ->
    impl Future<Output = Result<(), Self::Error>>;

    /// Create a new directory.
    ///
    /// If `parent_id` is unspecified, the directory is created at the
    /// filesystem root.
    fn create_folder(&self, parent_id: Option<&str>, name: &str) ->
    impl Future<Output = Result<Self::File, Self::Error>>;

    /// Create a new file.
    ///
    /// If `parent_id` is unspecified, the directory is created at the
    /// filesystem root.
    fn create_file(&self, parent_id: Option<&str>, name: &str) -> impl
    Future<Output = Result<Self::File, Self::Error>>;

    /// Lists all immediate files belonging to `parent_id`.
    ///
    /// If `parent_id` is unspecified, then it returns all files below
    /// the filesystem root.
    fn list_files(&self, parent_id: Option<&str>) ->
    impl Future<Output = Result<Vec<Self::File>, Self::Error>>;

    /// Write to a file in the filesystem.
    fn write_to_file(&self, id: &str, buf: &[u8]) ->
    impl Future<Output = Result<Self::File, Self::Error>>;

    /// Read from a file in the filesystem.
    fn read_from_file(&self, id: &str) ->
    impl Future<Output = Result<Vec<u8>, Self::Error>>;
}
