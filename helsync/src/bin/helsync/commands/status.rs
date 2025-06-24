use crate::database::{Database, Drive, Filesystem, App, CloudProvider};
use helsync::oauth2;
use helsync::fs;
use helsync::sync::Sync;
use super::errors::*;

use sqlx::Error as SqlxError;
use anyhow::Result;
use clap::Parser;

/// Group of commands for inspecting sync changes.
#[derive(Parser, Debug)]
pub struct StatusOpt {
    /// Name of drive to merge to.
    pub name: String,
}

impl StatusOpt {
    pub async fn run (&self, db: &Database) -> Result<()> {
        let mut conn = db.acquire().await?;
        let drive: Drive = sqlx::query_as("SELECT * FROM Drive WHERE name=?")
            .bind(&self.name)
            .fetch_one(&mut *conn).await
            .map_err(handle_not_found_err)?;

        if drive.token.is_none() {
            return Err(anyhow::anyhow!("drive has not been authenticated"));
        }

        let app: App = sqlx::query_as("SELECT * FROM App WHERE name=?")
            .bind(&drive.app)
            .fetch_one(&mut *conn)
            .await?;

        let app_config: oauth2::Config = app.clone().into();

        let local_fs: Filesystem = sqlx::query_as("SELECT * FROM Filesystem WHERE drive=?")
            .bind(&drive.name)
            .fetch_one(&mut *conn).await
            .map_err(|e| match e {
                SqlxError::RowNotFound => {
                    anyhow::anyhow!("no filesystem registered with drive")
                },
                _ => anyhow::anyhow!("{e}"),
            })?;

        // Create local root directory.
        std::fs::create_dir_all(&local_fs.path)
            .map_err(|_| anyhow::anyhow!("failed to create filesystem directory \"{}\"", &local_fs.path))?;

        // Acquire auth token.
        let auth_token = drive.token.unwrap();

        // Create remote filesystem.
        match app.provider {
            CloudProvider::OneDrive => {
                let client = fs::onedrive::OneDrive::new(&auth_token, &app_config);
                let db_path = std::path::PathBuf::from(&local_fs.path).join("db.sqlite");
                let sync = Sync::new(client, db_path.to_str().unwrap()).await?;
                let diff = sync.diff().await?;
                diff.iter().for_each(|f| println!("{f}"));
                Ok(())
            }
            CloudProvider::GoogleDrive => {
                let client = fs::googledrive::GoogleDrive::new(&auth_token, &app_config);
                Ok(())
            }
        }
    }
}
