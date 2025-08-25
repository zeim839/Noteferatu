#[cfg(feature = "plugin")]
const COMMANDS: &[&str] = &[
    "get_file",
    "copy_file",
    "move_file",
    "remove_file",
    "create_folder",
    "create_file",
    "list_files",
    "write_to_file",
    "read_from_file",
    "list_bookmarks",
    "create_bookmark",
    "remove_bookmark",
    "list_tags",
    "create_tag",
    "create_tag_bind",
    "remove_tag_bind",
];

fn main() {
    #[cfg(feature = "plugin")]
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
