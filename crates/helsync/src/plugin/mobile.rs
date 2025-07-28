use serde::de::DeserializeOwned;
use tauri::plugin::{PluginApi, PluginHandle};
use tauri::{AppHandle, Runtime};

use super::error::Result;
use super::models::*;
use crate::local::LocalFile;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_helsync);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> Result<Helsync<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("helsync", "Helsync")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_helsync)?;
    Ok(Helsync(handle))
}

/// Access to the helsync APIs.
pub struct Helsync<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Helsync<R> {
    pub async fn get_file(&self, payload: GetFileRequest) -> Result<LocalFile> {
        self.0.run_mobile_plugin("get_file", payload)
            .await.map_err(Into::into)
    }

    pub async fn copy_file(&self, payload: CopyFileRequest) -> Result<LocalFile> {
        self.0.run_mobile_plugin("copy_file", payload)
            .await.map_err(Into::into)
    }

    pub async fn move_file(&self, payload: MoveFileRequest) -> Result<LocalFile> {
        self.0.run_mobile_plugin("move_file", payload)
            .await.map_err(Into::into)
    }

    pub async fn remove_file(&self, payload: RemoveFileRequest) -> Result<()> {
        self.0.run_mobile_plugin("remove_file", payload)
            .await.map_err(Into::into)
    }

    pub async fn create_folder(&self, payload: CreateFolderRequest) -> Result<LocalFile> {
        self.0.run_mobile_plugin("create_folder", payload)
            .await.map_err(Into::into)
    }

    pub async fn create_file(&self, payload: CreateFileRequest) -> Result<LocalFile> {
        self.0.run_mobile_plugin("create_file", payload)
            .await.map_err(Into::into)
    }

    pub async fn list_files(&self, payload: ListFilesRequest) -> Result<Vec<LocalFile>> {
        self.0.run_mobile_plugin("list_files", payload)
            .await.map_err(Into::into)
    }

    pub async fn write_to_file(&self, payload: WriteToFileRequest) -> Result<LocalFile> {
        self.0.run_mobile_plugin("write_to_file", payload)
            .await.map_err(Into::into)
    }

    pub async fn read_from_file(&self, payload: GetFileRequest) -> Result<Vec<u8>> {
        self.0.run_mobile_plugin("read_from_file", payload)
            .await.map_err(Into::into)
    }
}
