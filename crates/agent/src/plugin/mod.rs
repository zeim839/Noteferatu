use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

pub use models::*;
pub use error::{Error, Result};

mod commands;
mod error;
mod models;

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
            #[cfg(mobile)]
            let agent = mobile::init(app, api)?;
            #[cfg(desktop)]
            let agent = desktop::init(app, api)?;
            app.manage(agent);
            Ok(())
        })
        .build()
}
