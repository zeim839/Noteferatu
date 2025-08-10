use agent::agent::SCHEMA_VERSION_0 as AGENT_SCHEMA_V0;
use helsync::local::SCHEMA_VERSION_0 as HELSYNC_SCHEMA_V0;
use std::sync::Arc;

fn app_db_dir() -> std::path::PathBuf {
    dirs::data_dir()
        .expect("could not resolve program data directory")
        .join("com.noteferatu.dev")
        .join("db.sqlite")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let db = Arc::new(database::Database::new(&database::Config {
        max_connections: 5,
        local_path: String::from(app_db_dir().to_str().unwrap()),
        migrations: vec![
            database::Migration {
                version: 0,
                sql: vec![HELSYNC_SCHEMA_V0, AGENT_SCHEMA_V0].concat(),
                kind: database::MigrationType::Up,
            },
        ],
    }).await.expect("could not initialize database"));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(macwindow::init())
        .plugin(agent::plugin::init(db.clone()))
        .plugin(helsync::plugin::init(db.clone()))
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
