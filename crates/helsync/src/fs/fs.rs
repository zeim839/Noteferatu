use anyhow::Result;

/// Common interface for interacting with different file systems.
pub trait FS {

    type File: File;
    type Delta: Delta;

    /// Reads the file with the given `id`.
    fn get_file(&self, id: &str) -> impl Future<Output = Result<Self::File>>;

    /// Copy the file with `source_id` to the parent with `parent_id`,
    /// optionally renaming it to `name`.
    fn copy_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) ->
    impl Future<Output = Result<Self::File>>;

    /// Move the file with `source_id` to the parent with `parent_id`,
    /// optionally renaming it to `name`.
    fn move_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) ->
    impl Future<Output = Result<Self::File>>;

    /// Delete the file with the given `id`.
    fn remove_file(&self, id: &str) -> impl Future<Output = Result<()>>;

    /// Create a new directory with name `name` at the parent
    /// `parent_id`. If `parent_id` is unspecified, the directory is
    /// created at the filesystem root.
    fn create_folder(&self, parent_id: Option<&str>, name: &str) ->
    impl Future<Output = Result<Self::File>>;

    /// Lists all immediate files belonging to `parent_id`. If
    /// `parent_id` is unspecified, then it returns all files below
    /// the filesystem root.
    fn list_files(&self, parent_id: Option<&str>) ->
    impl Future<Output = Result<Vec<Self::File>>>;

    /// Fetches the latest state of the filesystem. Specifying a
    /// `source_id` fetches changes only from given source. `token` is
    /// an optional token for omitting processed changes.
    fn track_changes(&self, parent_id: Option<&str>, token: Option<&str>) ->
    impl Future<Output = Result<(Vec<Self::Delta>, String)>>;

    /// Write to a file in the filesystem, creating it if it doesn't
    /// exist.
    fn write_to_file(&self, buf: &[u8], parent_id: Option<&str>, name: &str) ->
    impl Future<Output = Result<Self::File>>;

    /// Read from a file in the filesystem.
    fn read_from_file(&self, id: &str) -> impl Future<Output = Result<Vec<u8>>>;
}

/// Interface for exposing changes to [files](File).
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

/// Interface for exposing file metadata.
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
