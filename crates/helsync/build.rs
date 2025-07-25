#[cfg(feature = "plugin")]
const COMMANDS: &[&str] = &["ping"];

fn main() {
    #[cfg(feature = "plugin")]
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
