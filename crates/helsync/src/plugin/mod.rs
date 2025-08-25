//! [Tauri](tauri) plugin.
//!
//! The plugin module implements a [tauri] plugin that exposes the
//! Helsync crate functionality to Desktop and Mobile apps.

pub use crate::core::{Error, Result};

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

use database::Database;
use std::sync::Arc;

mod commands;

#[cfg(desktop)]
mod desktop;
#[cfg(desktop)]
use desktop::Helsync;

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
use mobile::Helsync;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and
/// [`tauri::Window`] to access the helsync APIs.
pub trait HelsyncExt<R: Runtime> {
    fn helsync(&self) -> &Helsync<R>;
}

impl<R: Runtime, T: Manager<R>> HelsyncExt<R> for T {
    fn helsync(&self) -> &Helsync<R> {
        self.state::<Helsync<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>(db: Arc<Database>) -> TauriPlugin<R> {
    Builder::new("helsync")
        .invoke_handler(tauri::generate_handler![
            commands::get_file,
            commands::copy_file,
            commands::move_file,
            commands::remove_file,
            commands::create_folder,
            commands::create_file,
            commands::list_files,
            commands::write_to_file,
            commands::read_from_file,
            commands::list_bookmarks,
            commands::create_bookmark,
            commands::remove_bookmark,
            commands::list_tags,
            commands::create_tag,
            commands::remove_tag,
            commands::change_tag_color,
            commands::create_tag_bind,
            commands::remove_tag_bind,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let helsync = mobile::init(app, api).unwrap();
            #[cfg(desktop)]
            let helsync = desktop::init(app, api, db).unwrap();
            app.manage(helsync);
            Ok(())
        })
        .build()
}
