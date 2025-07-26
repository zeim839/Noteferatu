use serde::de::DeserializeOwned;
use tauri::plugin::{PluginApi, PluginHandle};
use tauri::{AppHandle, Runtime};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_helsync);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Helsync<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("", "helsync")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_helsync)?;
    Ok(Helsync(handle))
}

/// Access to the helsync APIs.
pub struct Helsync<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Helsync<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        self.0.run_mobile_plugin("ping", payload)
            .map_err(Into::into)
    }
}
