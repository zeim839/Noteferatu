use serde::{Serialize, Deserialize};

/// File object metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {

    /// A unique identifier for the file.
    pub id: String,

    /// Unix timestamp of the last known modification.
    pub modified_at: i64,

    /// Unix timestamp of the file's creation.
    pub created_at: i64,

    /// ID of the file's parent.
    ///
    /// If None, then the parent is the root directory.
    pub parent_id: Option<String>,

    /// Whether the file object is a folder.
    pub is_folder: bool,
}
