use serde::de::DeserializeOwned;
use tauri::plugin::{PluginApi, PluginHandle};
use tauri::{AppHandle, Runtime};

use crate::models::*;
use crate::Result;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_agent);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Agent<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("", "AgentPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_agent)?;
    Ok(Agent(handle))
}

/// Access to the agent APIs.
pub struct Agent<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Agent<R> {
    pub fn try_connect(&self, payload: TryConnectRequest) -> Result<()> {
        self.0.run_mobile_plugin("try_connect", payload)
            .map_err(Into::into)
    }
}
