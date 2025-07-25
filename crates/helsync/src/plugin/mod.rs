//! [Tauri](tauri) plugin implementation.

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

mod commands;
mod error;
mod models;

pub use models::*;
pub use error::{Error, Result};

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
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("helsync")
        .invoke_handler(tauri::generate_handler![
            commands::ping
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let helsync = mobile::init(app, api)?;
            #[cfg(desktop)]
            let helsync = desktop::init(app, api)?;
            app.manage(helsync);
            Ok(())
        })
        .build()
}
