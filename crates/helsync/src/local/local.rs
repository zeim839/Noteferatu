use super::file::LocalFile;
use crate::filesystem::Filesystem;
use crate::onedrive::OneDrive;
use crate::googledrive::GoogleDrive;
use crate::errors::Result;

use std::time::{SystemTime, UNIX_EPOCH};
use sqlx::Acquire;
use database::Database;
use std::sync::Arc;

/// Local virtual filesystem.
///
/// Implements a local virtual filesystem using an SQLite
/// database. Optionally syncs with a remote
/// [OneDrive](crate::onedrive::OneDrive) or
/// [GoogleDrive](crate::googledrive::GoogleDrive) filesystem.
///
/// Cannot sync to multiple remote filesystems.
pub struct LocalFS {
    onedrive: Option<Arc<OneDrive>>,
    googledrive: Option<Arc<GoogleDrive>>,
    db: Database,
}

impl LocalFS {

    /// Initialize a new local filesystem.
    pub fn new(db: Database) -> Self {
        Self { db, onedrive: None, googledrive: None }
    }

    /// Bind a [GoogleDrive](crate::googledrive::GoogleDrive)
    /// filesystem to sync to.
    ///
    /// # Panics
    /// Panics if a [OneDrive](crate::onedrive::OneDrive) filesystem
    /// has already been registered.
    pub fn attach_googledrive(self, googledrive: GoogleDrive) -> Self {
        if self.onedrive.is_some() {
            panic!("cannot attach multiple filesystems");
        }
        Self { googledrive: Some(Arc::new(googledrive)), ..self }
    }

    /// Bind a [OneDrive](crate::onedrive::OneDrive)
    /// filesystem to sync to.
    ///
    /// # Panics
    /// Panics if a [GoogleDrive](crate::googledrive::GoogleDrive)
    /// filesystem has already been registered.
    pub fn attach_onedrive(self, onedrive: OneDrive) -> Self {
        if self.googledrive.is_some() {
            panic!("cannot attach multiple filesystems");
        }
        Self { onedrive: Some(Arc::new(onedrive)), ..self }
    }
}

impl Filesystem for LocalFS {
    type File = LocalFile;
    type Delta = LocalFile;

    /// Retrieve the file with the given `id`.
    async fn get_file(&self, id: &str) -> Result<Self::File> {
        let mut conn = self.db.acquire().await?;
        let file = sqlx::query_as("SELECT * FROM File WHERE id=? AND is_deleted=FALSE")
            .bind(id)
            .fetch_one(&mut *conn)
            .await?;

        Ok(file)
    }

    async fn copy_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) -> Result<Self::File> {
        let mut conn = self.db.acquire().await?;
        let mut tx = conn.begin().await?;
        let original: LocalFile = sqlx::query_as("SELECT * FROM File
        WHERE id=? AND is_deleted=FALSE")
            .bind(source_id)
            .fetch_one(&mut *tx)
            .await?;

        let name = match name {
            Some(name) => name.to_string(),
            None => original.name,
        };

        let created_at: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let parent_id: Option<String> = match parent_id {
            Some(parent_id) => Some(parent_id.to_string()),
            None => original.parent.map(|p| p.to_string()),
        };

        // Create copy.
        let res = sqlx::query("INSERT INTO File (name, parent,
        is_deleted, created_at, modified_at, is_folder) VALUES (?, ?,
        FALSE, ?, ?, ?)")
            .bind(name)
            .bind(parent_id)
            .bind(created_at)
            .bind(created_at)
            .bind(original.is_folder)
            .execute(&mut *tx)
            .await?;

        let copied_file: LocalFile = sqlx::query_as("SELECT * FROM
        File WHERE id=?")
            .bind(res.last_insert_rowid())
            .fetch_one(&mut *tx)
            .await?;

        // TODO Copy children.
        // if original.is_folder {
        //     let children: Vec<LocalFile> = sqlx::query_as("SELECT *
        // FROM File WHERE parent=? AND is_deleted=FALSE")
        //         .bind(source_id)
        //         .fetch_all(&mut *tx)
        //         .await?;

        //     for child in children {
        //         self.copy_file(
        //             &child.id.to_string(),
        //             Some(&original.id.to_string()),
        //             None
        //         ).await?;
        //     }
        // }

        tx.commit().await?;
        Ok(copied_file)
    }

    /// Move file to a new parent.
    async fn move_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) -> Result<Self::File> {
        let mut conn = self.db.acquire().await?;
        let mut tx = conn.begin().await?;
        match name {
            Some(name) =>
                sqlx::query("UPDATE FILE SET parent=?, name=? WHERE id=?")
                .bind(parent_id)
                .bind(name)
                .bind(source_id)
                .execute(&mut *tx)
                .await?,
            None =>
                sqlx::query("UPDATE File SET parent=? WHERE id=?")
                .bind(parent_id)
                .bind(source_id)
                .execute(&mut *tx)
                .await?,
        };

        let file: LocalFile = sqlx::query_as("SELECT * FROM File WHERE id=?")
            .bind(source_id)
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(file)
    }

    /// Delete the file with the given `id`.
    async fn remove_file(&self, id: &str) -> Result<()> {
        let mut conn = self.db.acquire().await?;
        let modified_at: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let res = sqlx::query("UPDATE File SET is_deleted=TRUE,
            modified_at=? WHERE is_deleted=FALSE AND (id=? OR parent=?)")
            .bind(modified_at)
            .bind(id)
            .bind(id)
            .execute(&mut *conn)
            .await?;

        if res.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound.into());
        }

        Ok(())
    }

    /// Create a new folder.
    async fn create_folder(&self, parent_id: Option<&str>, name: &str) -> Result<Self::File> {
        let mut conn = self.db.acquire().await?;
        let mut tx = conn.begin().await?;
        let created_at: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let res = sqlx::query("INSERT INTO File(name, parent,
        is_deleted, created_at, modified_at, is_folder) VALUES (?, ?,
        FALSE, ?, ?, TRUE)")
            .bind(name)
            .bind(parent_id)
            .bind(created_at)
            .bind(created_at)
            .execute(&mut *tx)
            .await?;

        let folder: LocalFile = sqlx::query_as("SELECT * FROM File WHERE id=?")
            .bind(res.last_insert_rowid())
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(folder)
    }

    async fn list_files(&self, parent_id: Option<&str>) -> Result<Vec<Self::File>> {
        let mut conn = self.db.acquire().await?;
        let files: Vec<LocalFile> = match parent_id {
            Some(parent) => sqlx::query_as("SELECT * FROM File WHERE
        is_deleted=FALSE AND parent=?")
                .bind(parent)
                .fetch_all(&mut *conn)
                .await?,
            None => sqlx::query_as("SELECT * FROM File WHERE
        is_deleted=FALSE AND parent IS NULL")
                .fetch_all(&mut *conn)
                .await?,
        };
        Ok(files)
    }

    async fn track_changes(&self, _: Option<&str>, _: Option<&str>) -> Result<(Vec<Self::Delta>, String)> {
        unimplemented!();
    }

    async fn write_to_file(&self, _: &[u8], _: Option<&str>, _: &str) -> Result<Self::File> {
        unimplemented!();
    }

    async fn read_from_file(&self, _: &str) -> Result<Vec<u8>> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {

    use crate::local::schema;
    use super::*;

    use std::sync::Arc;
    use tokio::sync::OnceCell;
    static FS: OnceCell<Arc<LocalFS>> = OnceCell::const_new();

    async fn get_local_fs() -> Arc<LocalFS> {
        FS.get_or_init(|| async {

            // Delete any existing test databases.
            let _ = std::fs::remove_file("./hs-localfs-test-db.sqlite");
            let _ = std::fs::remove_file("./hs-localfs-test-db.sqlite-shm");
            let _ = std::fs::remove_file("./hs-localfs-test-db.sqlite-wal");

            // Add files for testing.
            const TESTING_SCHEMA: &str = "
INSERT INTO File VALUES
  (0, \"test.txt\", NULL, NULL, FALSE, 0, 0, NULL, FALSE),
  (1, \"test-dltd\", NULL, NULL, TRUE, 0, 0, NULL, TRUE),
  (2, \"my_folder\", NULL, NULL, FALSE, 0, 0, NULL, TRUE),
  (3, \"move_me.txt\", NULL, NULL, FALSE, 0, 0, NULL, FALSE),
  (4, \"delete_me.txt\", NULL, NULL, FALSE, 0, 0, NULL, FALSE),
  (5, \"delete_me\", NULL, NULL, FALSE, 0, 0, NULL, TRUE),
  (6, \"deleted_child.txt\", 5, NULL, FALSE, 0, 0, NULL, FALSE),
  (7, \"test-list\", NULL, NULL, FALSE, 0, 0, NULL, TRUE),
  (8, \"list-child.txt\", 7, NULL, FALSE, 0, 0, NULL, FALSE),
  (9, \"test-list-deleted\", NULL, NULL, FALSE, 0, 0, NULL, TRUE),
  (10, \"list-child.txt\", 9, NULL, FALSE, 0, 0, NULL, FALSE);
";

            let db = database::Database::new(&database::Config {
                max_connections: 1,
                local_path: "./hs-localfs-test-db.sqlite".to_string(),
                migrations: vec![
                    database::Migration {
                        version: 0,
                        sql: schema::SCHEMA_VERSION_0,
                        kind: database::MigrationType::Up,
                    },
                    database::Migration {
                        version: 1,
                        sql: TESTING_SCHEMA,
                        kind: database::MigrationType::Up,
                    }
                ],
            }).await.unwrap();

            Arc::new(LocalFS::new(db))

        }).await.clone().clone()
    }

    #[tokio::test]
    async fn test_get_file() {
        let fs = get_local_fs().await;
        assert!(fs.get_file("0").await.is_ok());

        // Should not return deleted files.
        assert!(fs.get_file("1").await.is_err());
    }

    #[tokio::test]
    async fn test_copy_file() {
    }

    #[tokio::test]
    async fn test_move_file() {
        let fs = get_local_fs().await;
        let moved = fs.move_file("3", Some("2"), Some("renamed.txt"))
            .await.unwrap();

        assert!(moved.parent.is_some_and(|p| p == 2));
        assert!(moved.name == "renamed.txt");

        // Should not be able to move file to non-folder.
        assert!(fs.move_file("3", Some("0"), None).await.is_err());

        // Should not be able to move file to deleted folder.
        assert!(fs.move_file("3", Some("1"), None).await.is_err());
    }

    #[tokio::test]
    async fn test_remove_file() {
        let fs = get_local_fs().await;
        assert!(fs.remove_file("4").await.is_ok());
        assert!(fs.remove_file("5").await.is_ok());

        // Children of deleted files must also be deleted.
        assert!(fs.get_file("6").await.is_err());

        // Do not allow already-deleted files to be deleted again.
        assert!(fs.remove_file("1").await.is_err());
    }

    #[tokio::test]
    async fn test_create_folder() {
        let fs = get_local_fs().await;
        let folder = fs.create_folder(None, "created-folder")
            .await.unwrap();

        assert!(folder.name == "created-folder");
        assert!(folder.parent.is_none());

        // Do not allow creating folders under deleted folders.
        assert!(fs.create_folder(Some("1"), "invalid-folder")
                .await.is_err());

        // Do not allow creating folders under files.
        assert!(fs.create_folder(Some("0"), "invalid-folder")
                .await.is_err());
    }

    #[tokio::test]
    async fn test_list_files() {
        let fs = get_local_fs().await;
        let files = fs.list_files(None).await.unwrap();
        assert!(files.len() > 0);

        // Should only fetch first-level children.
        // No deleted members.
        files.iter().for_each(|item| {
            assert!(item.parent == None);
            assert!(!item.is_deleted);
        });

        // List files of subdirectory.
        let files = fs.list_files(Some("7")).await.unwrap();
        files.iter().for_each(|item| {
            assert!(item.parent.is_some_and(|p| p == 7));
            assert!(!item.is_deleted);
        });

        // Should return empty vector for deleted subdirectory.
        assert!(fs.remove_file("9").await.is_ok());
        let files = fs.list_files(Some("9")).await.unwrap();
        assert!(files.len() == 0);
    }

    #[tokio::test]
    async fn test_track_changes() {
    }

    #[tokio::test]
    async fn test_write_to_file() {
    }

    #[tokio::test]
    async fn test_read_from_file() {
    }
}
