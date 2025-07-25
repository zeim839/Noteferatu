//! [Tauri](tauri) plugin implementation.

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

mod commands;
mod error;
mod models;
mod schema;

#[cfg(desktop)]
mod desktop;
#[cfg(desktop)]
use desktop::Helsync;

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
use mobile::Helsync;

pub use models::*;
pub use error::{Error, Result};
use database::{Database, Config};

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
            commands::get_file,
            commands::copy_file,
            commands::move_file,
            commands::remove_file,
            commands::create_folder,
            commands::create_folder,
            commands::list_files,
        ])
        .setup(|app, api| {
            let setup = async || {
                let path = app.path().app_data_dir().unwrap()
                    .join("db.sqlite");

                let db = Database::new(&Config {
                    max_connections: 5,
                    local_path: String::from(path.to_str().unwrap()),
                    migrations: vec![schema::MIGRATION_V0],
                }).await.unwrap();

                #[cfg(mobile)]
                let helsync = mobile::init(app, api).unwrap();

                #[cfg(desktop)]
                let helsync = desktop::init(app, api, db).unwrap();

                app.manage(helsync);
            };

            if tokio::runtime::Handle::try_current().is_ok() {
                tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(setup()));
            } else {
                tauri::async_runtime::block_on(setup());
            }

            Ok(())
        })
        .build()
}
