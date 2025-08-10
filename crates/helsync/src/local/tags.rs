use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use super::file::LocalFile;

/// A tag organizes file entries under a common category (name).
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    pub color: String,
    pub created_at: i64,
}

/// Describes a binding of a [Tag] to a [File](super::LocalFile).
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TagBind {
    pub tag: String,
    pub file: i64,
}

/// A tag with all of its member files.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TagWithFiles {
    pub name: String,
    pub color: String,
    pub files: Vec<LocalFile>,
}
