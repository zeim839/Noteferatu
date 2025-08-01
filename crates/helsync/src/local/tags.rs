use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use super::file::LocalFile;

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    pub color: String,
    pub created_at: i64,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TagBind {
    pub tag: String,
    pub file: i64,
}

/// A tag with all of its associated files.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TagWithFiles {
    pub name: String,
    pub color: String,
    pub files: Vec<LocalFile>,
}