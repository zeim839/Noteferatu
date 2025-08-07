#[cfg(feature = "plugin")]
const COMMANDS: &[&str] = &[
    "try_connect",
    "list_models",
    "list_conversations",
    "create_conversation",
    "rename_conversation",
    "remove_conversation",
];

fn main() {
    #[cfg(feature = "plugin")]
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
