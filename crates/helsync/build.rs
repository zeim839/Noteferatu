#[cfg(feature = "plugin")]
const COMMANDS: &[&str] = &[
    "get_file",
    "copy_file",
    "move_file",
    "remove_file",
    "create_folder",
    "list_files"
];

fn main() {
    #[cfg(feature = "plugin")]
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
