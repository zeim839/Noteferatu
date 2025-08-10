use crate::local::{LocalFS, LocalFile, Tag, TagWithFiles};
use crate::filesystem::Filesystem;
use super::error::Result;

use std::sync::Arc;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use serde::de::DeserializeOwned;
use database::Database;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
    db: Arc<Database>,
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

    /// Retrieve the file with the given `id`.
    pub async fn get_file(&self, id: &str) -> Result<LocalFile> {
        Ok(self.local.clone().get_file(id).await?)
    }

    /// Recursively copy the file `source_id` to the folder
    /// `parent_id` with its name set to `name`.
    ///
    /// If `parent_id` is `None`, the file is copied to the root
    /// directory.
    pub async fn copy_file(
        &self,
        source_id: &str,
        parent_id: Option<&str>,
        name: Option<&str>
    ) -> Result<LocalFile> {
        Ok(self.local.copy_file(source_id, parent_id, name).await?)
    }

    /// Move file to a new parent.
    pub async fn move_file(
        &self,
        source_id: &str,
        parent_id: Option<&str>,
        name: Option<&str>
    ) -> Result<LocalFile> {
        Ok(self.local.move_file(source_id, parent_id, name).await?)
    }

    /// Delete the file with the given `id`.
    pub async fn remove_file(&self, id: &str) -> Result<()> {
        Ok(self.local.remove_file(id).await?)
    }

    /// Create a new folder.
    pub async fn create_folder(
        &self,
        parent_id: Option<&str>,
        name: &str
    ) -> Result<LocalFile> {
        Ok(self.local.create_folder(parent_id, name).await?)
    }

    /// Create a new file.
    pub async fn create_file(
        &self,
        parent_id: Option<&str>,
        name: &str
    ) -> Result<LocalFile> {
        Ok(self.local.create_file(parent_id, name).await?)
    }

    /// List files under a parent.
    pub async fn list_files(
        &self,
        parent_id: Option<&str>
    ) -> Result<Vec<LocalFile>> {
        Ok(self.local.list_files(parent_id).await?)
    }

    /// Write a slice of bytes to a file.
    ///
    /// The file's content will be replaced with the bytes.
    pub async fn write_to_file(
        &self,
        id: &str,
        contents: Vec<u8>
    ) -> Result<LocalFile> {
        Ok(self.local.write_to_file(id, &contents).await?)
    }

    /// Read the file's binary data.
    pub async fn read_from_file(&self) -> Result<()> {
        unimplemented!();
    }

    /// Fetch all bookmarked files.
    pub async fn list_bookmarks(&self) -> Result<Vec<LocalFile>> {
        Ok(self.local.list_bookmarks().await?)
    }

    /// Bookmark a file for convenient retrieval.
    pub async fn create_bookmark(&self, id: &str) -> Result<()> {
        Ok(self.local.create_bookmark(id).await?)
    }

    /// Removes a bookmark from a file.
    pub async fn remove_bookmark(&self, id: &str) -> Result<()> {
        Ok(self.local.remove_bookmark(id).await?)
    }

    /// List all available tags, including those with no associated files.
    pub async fn list_tags(&self) -> Result<Vec<TagWithFiles>> {
        Ok(self.local.list_tags().await?)
    }

    /// Create a new tag.
    pub async fn create_tag(
        &self,
        name: &str,
        color: &str
    ) -> Result<Tag> {
        Ok(self.local.create_tag(name, color).await?)
    }

    /// Remove a tag and all its tag binds.
    pub async fn remove_tag(
        &self,
        name: &str
    ) -> Result<()> {
        Ok(self.local.remove_tag(name).await?)
    }

    /// Change a tag's color.
    pub async fn change_tag_color(
        &self,
        name: &str,
        color: &str
    ) -> Result<()> {
        Ok(self.local.change_tag_color(name, color).await?)
    }

    /// Attach a tag to a file.
    pub async fn create_tag_bind(
        &self,
        file_id: &str,
        tag_name: &str
    ) -> Result<()> {
        Ok(self.local.create_tag_bind(file_id, tag_name).await?)
    }

    /// Remove a tag from a file.
    pub async fn remove_tag_bind(
        &self,
        file_id: &str,
        tag_name: &str
    ) -> Result<()> {
        Ok(self.local.remove_tag_bind(file_id, tag_name).await?)
    }
}
