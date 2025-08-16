use crate::local::{Tag, TagWithFiles, LocalFile};
use crate::core::Result;
use super::HelsyncExt;

use tauri::{AppHandle, command, Runtime, Emitter};

#[command]
pub(crate) async fn get_file<R: Runtime>(
    app: AppHandle<R>,
    id: &str
) -> Result<LocalFile> {
    app.helsync().get_file(id).await
}

#[command]
pub(crate) async fn copy_file<R: Runtime>(
    app: AppHandle<R>,
    source_id: &str,
    parent_id: Option<&str>,
    name: Option<&str>
) -> Result<LocalFile> {
    app.emit("helsync-fs-change", "")?;
    app.helsync().copy_file(source_id, parent_id, name).await
}

#[command]
pub(crate) async fn move_file<R: Runtime>(
    app: AppHandle<R>,
    source_id: &str,
    parent_id: Option<&str>,
    name: Option<&str>
) -> Result<LocalFile> {
    app.emit("helsync-fs-change", "")?;
    app.helsync().move_file(source_id, parent_id, name).await
}

#[command]
pub(crate) async fn remove_file<R: Runtime>(
    app: AppHandle<R>,
    id: &str
) -> Result<()> {
    app.emit("helsync-fs-change", "")?;
    app.helsync().remove_file(id).await
}

#[command]
pub(crate) async fn create_folder<R: Runtime>(
    app: AppHandle<R>,
    parent_id: Option<&str>,
    name: &str
) -> Result<LocalFile> {
    app.emit("helsync-fs-change", "")?;
    app.helsync().create_folder(parent_id, name).await
}

#[command]
pub(crate) async fn create_file<R: Runtime>(
    app: AppHandle<R>,
    parent_id: Option<&str>,
    name: &str
) -> Result<LocalFile> {
    app.emit("helsync-fs-change", "")?;
    app.helsync().create_file(parent_id, name).await
}

#[command]
pub(crate) async fn list_files<R: Runtime>(
    app: AppHandle<R>,
    parent_id: Option<&str>
) -> Result<Vec<LocalFile>> {
    app.helsync().list_files(parent_id).await
}

#[command]
pub(crate) async fn write_to_file<R: Runtime>(
    app: AppHandle<R>,
    id: &str,
    contents: Vec<u8>
) -> Result<LocalFile> {
    app.emit("helsync-fs-change", "")?;
    app.helsync().write_to_file(id, contents).await
}

#[command]
pub(crate) async fn list_bookmarks<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<LocalFile>> {
    app.helsync().list_bookmarks().await
}

#[command]
pub(crate) async fn create_bookmark<R: Runtime>(
    app: AppHandle<R>,
    id: &str
) -> Result<()> {
    app.emit("helsync-bookmark-change", "")?;
    app.helsync().create_bookmark(id).await
}

#[command]
pub(crate) async fn remove_bookmark<R: Runtime>(
    app: AppHandle<R>,
    id: &str
) -> Result<()> {
    app.emit("helsync-bookmark-change", "")?;
    app.helsync().remove_bookmark(id).await
}

#[command]
pub(crate) async fn list_tags<R: Runtime>(
    app: AppHandle<R>
) -> Result<Vec<TagWithFiles>> {
    app.helsync().list_tags().await
}

#[command]
pub(crate) async fn create_tag<R: Runtime>(
    app: AppHandle<R>,
    name: &str,
    color: &str
) -> Result<Tag> {
    app.emit("helsync-tags-change", "")?;
    app.helsync().create_tag(name, color).await
}

#[command]
pub(crate) async fn remove_tag<R: Runtime>(
    app: AppHandle<R>,
    name: &str,
) -> Result<()> {
    app.emit("helsync-tags-change", "")?;
    app.helsync().remove_tag(name).await
}

#[command]
pub(crate) async fn change_tag_color<R: Runtime>(
    app: AppHandle<R>,
    name: &str,
    color: &str
) -> Result<()> {
    app.emit("helsync-tags-change", "")?;
    app.helsync().change_tag_color(name, color).await
}

#[command]
pub(crate) async fn create_tag_bind<R: Runtime>(
    app: AppHandle<R>,
    file_id: &str,
    tag_name: &str
) -> Result<()> {
    app.emit("helsync-tags-change", "")?;
    app.helsync().create_tag_bind(file_id, tag_name).await
}

#[command]
pub(crate) async fn remove_tag_bind<R: Runtime>(
    app: AppHandle<R>,
    file_id: &str,
    tag_name: &str
) -> Result<()> {
    app.emit("helsync-tags-change", "")?;
    app.helsync().remove_tag_bind(file_id, tag_name).await
}
