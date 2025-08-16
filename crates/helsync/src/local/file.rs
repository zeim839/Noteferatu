use serde::{Serialize, Deserialize};
use crate::core::File;
use sqlx::FromRow;

/// LocalFile is a local [FileSystem](crate::core::FileSystem) file.
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocalFile {
    pub id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    pub is_deleted: bool,
    pub created_at: i64,
    pub modified_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<i64>,
    pub is_folder: bool,
    pub is_bookmarked: bool,
}

impl Into<File> for LocalFile {
    fn into(self) -> File {
        File {
            id: self.id.to_string(),
            modified_at: self.modified_at,
            created_at: self.created_at,
            parent_id: self.parent.map(|p| p.to_string()),
            is_folder: self.is_folder,
        }
    }
}
