use tauri::plugin::{Builder, TauriPlugin};
use tauri::Runtime;

#[cfg(target_os = "macos")]
mod macos;

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("mac-window")
        .on_window_ready(|window| {
            #[cfg(target_os = "macos")]
            {
                macos::setup_traffic_light_positioner(&window);
            }
        })
        .build()
}
