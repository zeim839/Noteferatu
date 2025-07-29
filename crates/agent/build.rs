#[cfg(feature = "plugin")]
const COMMANDS: &[&str] = &["try_connect", "list_models"];

fn main() {
    #[cfg(feature = "plugin")]
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
