use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

use database::{Database, Config, Migration, MigrationType};
use crate::agent::SCHEMA_VERSION_0;
use std::sync::Arc;

mod commands;
mod models;
pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

#[cfg(desktop)]
use desktop::Agent;
#[cfg(mobile)]
use mobile::Agent;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the agent APIs.
pub trait AgentExt<R: Runtime> {
    fn agent(&self) -> &Agent<R>;
}

impl <R: Runtime, T: Manager<R>> AgentExt<R> for T {
    fn agent(&self) -> &Agent<R> {
        self.state::<Agent<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("agent")
        .invoke_handler(tauri::generate_handler![
            commands::try_connect,
            commands::list_models,
        ])
        .setup(|app, api| {
            let setup = async || {
                let path = app.path().app_data_dir().unwrap()
                    .join("temp.sqlite");

                let db = Database::new(&Config {
                    max_connections: 5,
                    local_path: String::from(path.to_str().unwrap()),
                    migrations: vec![Migration {
                        version: 0,
                        sql: SCHEMA_VERSION_0,
                        kind: MigrationType::Up,
                    }],
                }).await.unwrap();

                #[cfg(mobile)]
                let agent = mobile::init(app, api, db).unwrap();

                #[cfg(desktop)]
                let agent = desktop::init(app, api, Arc::new(db)).unwrap();

                app.manage(agent);
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
