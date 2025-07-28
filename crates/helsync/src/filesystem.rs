use crate::errors::Result;
use std::future::Future;

/// Common filesystem interface.
pub trait Filesystem {
    type File: File;
    type Delta: Delta;

    /// Fetches metadata for the file with `id`.
    fn get_file(&self, id: &str) -> impl Future<Output = Result<Self::File>>;

    /// Copy a file.
    ///
    /// Copies a file and moves it under `parent_id`. If `parent_id` is
    /// unspecified, then the file is moved to the root directory. If
    /// `name` is specified, the file is renamed.
    fn copy_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) ->
    impl Future<Output = Result<Self::File>>;

    /// Move a file.
    ///
    /// Sets the file's parent to `parent_id`. If `parent_id` is
    /// unspecified, then the file is moved to the root directory. If
    /// `name` is specified, the file is renamed.
    fn move_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) ->
    impl Future<Output = Result<Self::File>>;

    /// Delete the file with the given `id`.
    fn remove_file(&self, id: &str) -> impl Future<Output = Result<()>>;

    /// Create a new directory.
    ///
    /// If `parent_id` is unspecified, the directory is created at the
    /// filesystem root.
    fn create_folder(&self, parent_id: Option<&str>, name: &str) ->
    impl Future<Output = Result<Self::File>>;

    /// Create a new file.
    ///
    /// If `parent_id` is unspecified, the directory is created at the
    /// filesystem root.
    fn create_file(&self, parent_id: Option<&str>, name: &str) -> impl
    Future<Output = Result<Self::File>>;

    /// Lists all immediate files belonging to `parent_id`.
    ///
    /// If `parent_id` is unspecified, then it returns all files below
    /// the filesystem root.
    fn list_files(&self, parent_id: Option<&str>) ->
    impl Future<Output = Result<Vec<Self::File>>>;

    /// Fetches the latest state of the filesystem.
    ///
    /// Specifying a `source_id` fetches changes only from given
    /// source. `token` is an optional token for omitting processed
    /// changes.
    fn track_changes(&self, parent_id: Option<&str>, token: Option<&str>) ->
    impl Future<Output = Result<(Vec<Self::Delta>, String)>>;

    /// Write to a file in the filesystem.
    fn write_to_file(&self, id: &str, buf: &[u8]) ->
    impl Future<Output = Result<Self::File>>;

    /// Read from a file in the filesystem.
    fn read_from_file(&self, id: &str) -> impl Future<Output = Result<Vec<u8>>>;
}

/// [File] changes.
pub trait Delta {

    /// ID of the associated [File].
    fn id(&self) -> String;

    /// Returns whether this file has been deleted.
    fn is_removed(&self) -> bool;

    /// Returns whether this file has been modified.
    ///
    /// "Modified" may mean a modification to its metadata (e.g. move
    /// to a new parent) or its content.
    fn is_modified(&self) -> bool;
}

/// File metadata.
pub trait File {

    /// A unique identifier for the file.
    fn id(&self) -> String;

    /// Unix timestamp of the last known modification.
    fn modified_at(&self) -> i64;

    /// Unix timestamp of the file's creation.
    fn created_at(&self) -> i64;

    /// Returns whether this item is a folder. If not a folder, then
    /// the item is a file.
    fn is_folder(&self) -> bool;

    /// Retrieves the ID of the file's parent. If None, then the
    /// parent is the root directory.
    fn parent(&self) -> Option<String>;
}
