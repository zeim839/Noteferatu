//! [Tauri](tauri) plugin.
//!
//! The [plugin](crate::plugin) module implements a [tauri] plugin
//! that exposes the Agent crate functionality to Desktop and Mobile
//! apps.

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

use database::Database;
use std::sync::Arc;

mod commands;
mod models;
pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

#[cfg(desktop)]
pub use desktop::Agent;
#[cfg(mobile)]
pub use mobile::Agent;

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
pub fn init<R: Runtime>(db: Arc<Database>) -> TauriPlugin<R> {
    Builder::new("agent")
        .invoke_handler(tauri::generate_handler![
            commands::try_connect,
            commands::list_models,
            commands::list_conversations,
            commands::create_conversation,
            commands::rename_conversation,
            commands::remove_conversation,
            commands::send_message,
            commands::send_stream_message,
            commands::update_message,
            commands::list_messages,
            commands::stop_messages,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let agent = mobile::init(app, api, db).unwrap();
            #[cfg(desktop)]
            let agent = desktop::init(app, api, db).unwrap();
            app.manage(agent);
            Ok(())
        })
        .build()
}
