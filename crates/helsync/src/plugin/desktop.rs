use crate::local::{LocalFS, LocalFile, Tag, TagWithFiles};
use crate::filesystem::Filesystem;
use super::models::*;
use super::error::Result;

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

    pub async fn create_file(&self, payload: CreateFileRequest) -> Result<LocalFile> {
        Ok(self.local.create_file(
            payload.parent_id.as_deref(),
            &payload.name,
        ).await?)
    }

    pub async fn list_files(&self, payload: ListFilesRequest) -> Result<Vec<LocalFile>> {
        Ok(self.local.list_files(payload.parent_id.as_deref()).await?)
    }

    pub async fn write_to_file(&self, payload: WriteToFileRequest) -> Result<LocalFile> {
        Ok(self.local.write_to_file(&payload.id, &payload.contents).await?)
    }

    pub async fn read_from_file(&self) -> Result<()> {
        unimplemented!();
    }

    pub async fn list_bookmarks(&self) -> Result<Vec<LocalFile>> {
        Ok(self.local.list_bookmarks().await?)
    }

    pub async fn create_bookmark(&self, payload: BookmarkRequest) -> Result<()> {
        Ok(self.local.create_bookmark(&payload.id).await?)
    }

    pub async fn remove_bookmark(&self, payload: BookmarkRequest) -> Result<()> {
        Ok(self.local.remove_bookmark(&payload.id).await?)
    }

    pub async fn list_tags(&self) -> Result<Vec<TagWithFiles>> {
        Ok(self.local.list_tags().await?)
    }

    pub async fn create_tag(&self, payload: CreateTagRequest) -> Result<Tag> {
        Ok(self.local.create_tag(&payload.name, &payload.color).await?)
    }

    pub async fn create_tag_bind(&self, payload: CreateTagBindRequest) -> Result<()> {
        Ok(self.local.create_tag_bind(&payload.file_id, &payload.tag_name).await?)
    }

    pub async fn remove_tag_bind(&self, payload: RemoveTagBindRequest) -> Result<()> {
        Ok(self.local.remove_tag_bind(&payload.file_id, &payload.tag_name).await?)
    }
}
