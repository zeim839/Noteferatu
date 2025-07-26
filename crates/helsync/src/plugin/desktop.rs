use super::models::*;
use super::error::Result;
use crate::local::{LocalFS, LocalFile};
use crate::filesystem::Filesystem;

use std::sync::Arc;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use serde::de::DeserializeOwned;
use database::Database;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
    db: Database,
) -> super::Result<Helsync<R>> {
    Ok(Helsync {
        _app: app.clone(),
        local: Arc::new(LocalFS::new(db)),
    })
}

/// Access to the helsync APIs.
pub struct Helsync<R: Runtime> {
    _app: AppHandle<R>,
    local: Arc<LocalFS>,
}

impl<R: Runtime> Helsync<R> {
    pub async fn get_file(&self, payload: GetFileRequest) -> Result<LocalFile> {
        Ok(self.local.clone().get_file(&payload.id).await?)
    }

    pub async fn copy_file(&self, payload: CopyFileRequest) -> Result<LocalFile> {
        Ok(self.local.clone().copy_file(
            &payload.source_id,
            payload.parent_id.as_deref(),
            payload.name.as_deref(),
        ).await?)
    }

    pub async fn move_file(&self, payload: MoveFileRequest) -> Result<LocalFile> {
        Ok(self.local.move_file(
            &payload.source_id,
            payload.parent_id.as_deref(),
            payload.name.as_deref(),
        ).await?)
    }

    pub async fn remove_file(&self, payload: RemoveFileRequest) -> Result<()> {
        Ok(self.local.remove_file(&payload.id).await?)
    }

    pub async fn create_folder(&self, payload: CreateFolderRequest) -> Result<LocalFile> {
        Ok(self.local.create_folder(
            payload.parent_id.as_deref(),
            &payload.name
        ).await?)
    }

    pub async fn list_files(&self, payload: ListFilesRequest) -> Result<Vec<LocalFile>> {
        Ok(self.local.list_files(payload.parent_id.as_deref()).await?)
    }

    pub async fn write_to_file(&self) -> Result<()> {
        unimplemented!();
    }

    pub async fn read_from_file(&self) -> Result<()> {
        unimplemented!();
    }
}
