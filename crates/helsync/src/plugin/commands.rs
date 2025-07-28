use tauri::{AppHandle, command, Runtime, Emitter};

use crate::local::LocalFile;
use super::models::*;
use super::Result;
use super::HelsyncExt;

#[command]
pub(crate) async fn get_file<R: Runtime>(
    app: AppHandle<R>,
    payload: GetFileRequest,
) -> Result<LocalFile> {
    app.helsync().get_file(payload).await
}

#[command]
pub(crate) async fn copy_file<R: Runtime>(
    app: AppHandle<R>,
    payload: CopyFileRequest,
) -> Result<LocalFile> {
    app.emit("helsync-fs-change", FsChangeEvent::Copy(payload.clone()))?;
    app.helsync().copy_file(payload).await
}

#[command]
pub(crate) async fn move_file<R: Runtime>(
    app: AppHandle<R>,
    payload: MoveFileRequest,
) -> Result<LocalFile> {
    app.emit("helsync-fs-change", FsChangeEvent::Move(payload.clone()))?;
    app.helsync().move_file(payload).await
}

#[command]
pub(crate) async fn remove_file<R: Runtime>(
    app: AppHandle<R>,
    payload: RemoveFileRequest,
) -> Result<()> {
    app.emit("helsync-fs-change", FsChangeEvent::Remove(payload.clone()))?;
    app.helsync().remove_file(payload).await
}

#[command]
pub(crate) async fn create_folder<R: Runtime>(
    app: AppHandle<R>,
    payload: CreateFolderRequest,
) -> Result<LocalFile> {
    app.emit("helsync-fs-change", FsChangeEvent::CreateFolder(payload.clone()))?;
    app.helsync().create_folder(payload).await
}

#[command]
pub(crate) async fn create_file<R: Runtime>(
    app: AppHandle<R>,
    payload: CreateFileRequest,
) -> Result<LocalFile> {
    app.emit("helsync-fs-change", FsChangeEvent::CreateFile(payload.clone()))?;
    app.helsync().create_file(payload).await
}

#[command]
pub(crate) async fn list_files<R: Runtime>(
    app: AppHandle<R>,
    payload: ListFilesRequest,
) -> Result<Vec<LocalFile>> {
    app.helsync().list_files(payload).await
}

#[command]
pub(crate) async fn write_to_file<R: Runtime>(
    app: AppHandle<R>,
    payload: WriteToFileRequest,
) -> Result<LocalFile> {
    // TODO: emit fs-change event
    app.helsync().write_to_file(payload).await
}
