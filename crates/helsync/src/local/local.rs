use super::file::LocalFile;
use crate::filesystem::Filesystem;
use crate::onedrive::OneDrive;
use crate::googledrive::GoogleDrive;
use crate::errors::Result;

use database::Database;
use std::sync::Arc;

pub struct LocalFS {
    onedrive: Option<Arc<OneDrive>>,
    googledrive: Option<Arc<GoogleDrive>>,
    db: Database,
}

impl LocalFS {

    /// Initialize a new local filesystem.
    pub async fn new(db: Database) -> Self {
        Self { db, onedrive: None, googledrive: None }
    }

    /// Bind a [GoogleDrive](crate::googledrive::GoogleDrive)
    /// filesystem to sync to.
    pub async fn attach_googledrive(self, googledrive: GoogleDrive) -> Self {
        if self.onedrive.is_some() {
            panic!("cannot attach multiple filesystems");
        }
        Self { googledrive: Some(Arc::new(googledrive)), ..self }
    }

    /// Bind a [OneDrive](crate::onedrive::OneDrive)
    /// filesystem to sync to.
    pub async fn attach_onedrive(self, onedrive: OneDrive) -> Self {
        if self.googledrive.is_some() {
            panic!("cannot attach multiple filesystems");
        }
        Self { onedrive: Some(Arc::new(onedrive)), ..self }
    }
}

impl Filesystem for LocalFS {
    type File = LocalFile;
    type Delta = LocalFile;

    async fn get_file(&self, id: &str) -> Result<Self::File> {
        unimplemented!();
    }

    async fn copy_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) -> Result<Self::File> {
        unimplemented!();
    }

    async fn move_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) -> Result<Self::File> {
        unimplemented!();
    }

    async fn remove_file(&self, id: &str) -> Result<()> {
        unimplemented!();
    }

    async fn create_folder(&self, parent_id: Option<&str>, name: &str) -> Result<Self::File> {
        unimplemented!();
    }

    async fn list_files(&self, parent_id: Option<&str>) -> Result<Vec<Self::File>> {
        unimplemented!();
    }

    async fn track_changes(&self, parent_id: Option<&str>, token: Option<&str>) -> Result<(Vec<Self::Delta>, String)> {
        unimplemented!();
    }

    async fn write_to_file(&self, buf: &[u8], parent_id: Option<&str>, name: &str) -> Result<Self::File> {
        unimplemented!();
    }

    async fn read_from_file(&self, id: &str) -> Result<Vec<u8>> {
        unimplemented!();
    }
}
