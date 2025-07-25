use crate::filesystem::{Delta, File};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct LocalFile {
    pub id: i64,
    pub name: String,
    pub parent: Option<i64>,
    pub remote_id: Option<String>,
    pub is_deleted: bool,
    pub created_at: i64,
    pub modified_at: i64,
    pub synced_at: Option<i64>,
    pub is_folder: bool,
}

impl File for LocalFile {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn modified_at(&self) -> i64 {
        self.modified_at
    }

    fn created_at(&self) -> i64 {
        self.created_at
    }

    fn is_folder(&self) -> bool {
        self.is_folder
    }

    fn parent(&self) -> Option<String> {
        self.parent.map(|p| p.to_string())
    }
}

impl Delta for LocalFile {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn is_removed(&self) -> bool {
        self.is_deleted
    }

    fn is_modified(&self) -> bool {
        if let Some(synced_at) = self.synced_at {
            return !self.is_deleted && synced_at > self.modified_at;
        }
        return false;
    }
}
