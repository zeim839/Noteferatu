use crate::core::{FileSystem, Delta, Result, Error};
use super::tags::{Tag, TagWithFiles};
use super::file::LocalFile;

use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use database::Database;
use std::sync::Arc;
use sqlx::Acquire;

/// Local virtual [FileSystem](crate::core::FileSystem).
pub struct Client {
    db: Arc<Database>
}

impl Client {

    /// Initialize a new local filesystem.
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Fetch all bookmarked files.
    pub async fn list_bookmarks(&self) -> Result<Vec<LocalFile>> {
        let mut conn = self.db.acquire().await?;
        let bookmarks: Vec<LocalFile> = sqlx::query_as("SELECT * FROM
    File WHERE is_bookmarked=TRUE AND is_deleted=FALSE")
            .fetch_all(&mut *conn)
            .await?;

        Ok(bookmarks)
    }

    /// Bookmark a file for convenient retrieval.
    pub async fn create_bookmark(&self, file_id: &str) -> Result<()> {
        let mut conn = self.db.acquire().await?;
        let res = sqlx::query("UPDATE File SET is_bookmarked=TRUE WHERE
    id=? AND is_deleted=FALSE AND is_bookmarked=FALSE")
            .bind(file_id)
            .execute(&mut *conn)
            .await?;

        if res.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound.into());
        }

        Ok(())
    }

    /// Removes a bookmark from a file.
    pub async fn remove_bookmark(&self, file_id: &str) -> Result<()> {
        let mut conn = self.db.acquire().await?;
        let res = sqlx::query("UPDATE File SET is_bookmarked=FALSE WHERE
    id=? AND is_deleted=FALSE AND is_bookmarked=TRUE")
            .bind(file_id)
            .execute(&mut *conn)
            .await?;

        if res.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound.into());
        }

        Ok(())
    }

    /// List all available tags, including those with no associated files.
    pub async fn list_tags(&self) -> Result<Vec<TagWithFiles>> {
        let mut conn = self.db.acquire().await?;
        let all_tags: Vec<Tag> = sqlx::query_as("SELECT * FROM Tag")
            .fetch_all(&mut *conn)
            .await?;

        let mut tags_map: HashMap<String, TagWithFiles> = all_tags
            .into_iter().map(|tag| {
                let tag_with_files = TagWithFiles {
                    name: tag.name.clone(),
                    color: tag.color,
                    files: Vec::new(),
                };
                (tag.name, tag_with_files)
            }).collect();

        #[derive(sqlx::FromRow, Debug)]
        struct TagFileRow {
            tag: String,
            #[sqlx(flatten)]
            file: LocalFile,
        }

        let rows: Vec<TagFileRow> = sqlx::query_as("SELECT TB.tag, F.*
        FROM TagBind TB JOIN File F ON TB.file=F.id WHERE
        F.is_deleted=FALSE")
            .fetch_all(&mut *conn)
            .await?;

        for row in rows {
            if let Some(tag_with_files) = tags_map.get_mut(&row.tag) {
                tag_with_files.files.push(row.file);
            }
        }

        let mut tags: Vec<TagWithFiles> = tags_map.into_values().collect();
        tags.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(tags)
    }

    /// Create a new tag.
    pub async fn create_tag(&self, name: &str, color: &str) -> Result<Tag> {
        let mut conn = self.db.acquire().await?;
        let created_at: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        sqlx::query("INSERT INTO Tag VALUES (?, ?, ?)")
            .bind(name)
            .bind(color)
            .bind(created_at)
            .execute(&mut *conn)
            .await?;

        Ok(Tag{
            name: name.to_string(),
            color: color.to_string(),
            created_at,
        })
    }

    /// Remove a tag and all its tag binds.
    pub async fn remove_tag(&self, name: &str) -> Result<()> {
        let mut conn = self.db.acquire().await?;
        let res = sqlx::query("DELETE FROM Tag WHERE name=?")
            .bind(name)
            .execute(&mut *conn)
            .await?;

        if res.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound.into());
        }

        Ok(())
    }

    /// Change a tag's color.
    pub async fn change_tag_color(&self, name: &str, color: &str) -> Result<()> {
        let mut conn = self.db.acquire().await?;
        let res = sqlx::query("UPDATE Tag SET color=? WHERE name=?")
            .bind(color)
            .bind(name)
            .execute(&mut *conn)
            .await?;

        if res.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound.into());
        }

        Ok(())
    }

    /// Attach a tag to a file.
    pub async fn create_tag_bind(&self, file_id: &str, tag_name: &str) -> Result<()> {
        let mut conn = self.db.acquire().await?;
        sqlx::query("INSERT INTO TagBind VALUES (?, ?)")
            .bind(tag_name)
            .bind(file_id)
            .execute(&mut *conn)
            .await?;

        Ok(())
    }

    /// Remove a tag from a file.
    pub async fn remove_tag_bind(&self, file_id: &str, tag_name: &str) -> Result<()> {
        let mut conn = self.db.acquire().await?;
        sqlx::query("DELETE FROM TagBind WHERE tag=? AND file=?")
            .bind(tag_name)
            .bind(file_id)
            .execute(&mut *conn)
            .await?;

        Ok(())
    }
}

impl FileSystem for Client {
    type File = LocalFile;
    type Error = Error;

    /// Retrieve the file with the given `id`.
    async fn get_file(&self, id: &str) -> Result<LocalFile> {
        let mut conn = self.db.acquire().await?;
        let file = sqlx::query_as("SELECT * FROM File WHERE id=? AND
 is_deleted=FALSE")
            .bind(id)
            .fetch_one(&mut *conn)
            .await?;

        Ok(file)
    }

    /// Recursively copy the file `source_id` to the folder
    /// `parent_id` with its name set to `name`.
    ///
    /// If `parent_id` is `None`, the file is copied to the root
    /// directory.
    async fn copy_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) -> Result<LocalFile> {
        let mut conn = self.db.acquire().await?;
        let mut tx = conn.begin().await?;
        let original: LocalFile = sqlx::query_as("SELECT * FROM File
        WHERE id=? AND is_deleted=FALSE")
            .bind(source_id)
            .fetch_one(&mut *tx)
            .await?;

        let name = match name {
            Some(name) => name.to_string(),
            None => original.name.clone(),
        };

        let created_at: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let new_parent_id: Option<String> = match parent_id {
            Some(parent_id) => Some(parent_id.to_string()),
            None => original.parent.map(|p| p.to_string()),
        };

        // Create root copy.
        let res = sqlx::query("INSERT INTO File (name, parent,
        is_deleted, created_at, modified_at, is_folder) VALUES (?, ?,
        FALSE, ?, ?, ?)")
            .bind(name)
            .bind(new_parent_id)
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

        if !original.is_folder {
            tx.commit().await?;
            return Ok(copied_file);
        }

        // Recursively copy children using a stack for DFS traversal.
        let mut stack: Vec<(i64, i64)> = vec![(original.id, copied_file.id)];
        while let Some((old_parent_id, new_parent_id)) = stack.pop() {
            let children: Vec<LocalFile> = sqlx::query_as("SELECT *
        FROM File WHERE parent=? AND is_deleted=FALSE")
                .bind(old_parent_id)
                .fetch_all(&mut *tx)
                .await?;

            for child in children {
                let res = sqlx::query("INSERT INTO File (name, parent,
                is_deleted, created_at, modified_at, is_folder) VALUES
                (?, ?, FALSE, ?, ?, ?)")
                    .bind(&child.name)
                    .bind(new_parent_id)
                    .bind(created_at)
                    .bind(created_at)
                    .bind(child.is_folder)
                    .execute(&mut *tx)
                    .await?;

                if child.is_folder {
                    stack.push((child.id, res.last_insert_rowid()));
                }
            }
        }

        tx.commit().await?;
        Ok(copied_file)
    }

    /// Move file to a new parent.
    async fn move_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) -> Result<LocalFile> {
        let mut conn = self.db.acquire().await?;
        let mut tx = conn.begin().await?;
        let modified_at: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        match name {
            Some(name) =>
                sqlx::query("UPDATE FILE SET parent=?, name=?, modified_at=? WHERE id=?")
                .bind(parent_id)
                .bind(name)
                .bind(modified_at)
                .bind(source_id)
                .execute(&mut *tx)
                .await?,
            None =>
                sqlx::query("UPDATE File SET parent=?, modified_at=? WHERE id=?")
                .bind(parent_id)
                .bind(modified_at)
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
    async fn create_folder(&self, parent_id: Option<&str>, name: &str) -> Result<LocalFile> {
        let mut conn = self.db.acquire().await?;
        let mut tx = conn.begin().await?;
        let created_at: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let res = sqlx::query("INSERT INTO File(name, parent,
        is_deleted, created_at, modified_at, is_folder, is_bookmarked)
        VALUES (?, ?, FALSE, ?, ?, TRUE, FALSE)")
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

        /// Create a new file.
    async fn create_file(&self, parent_id: Option<&str>, name: &str) -> Result<LocalFile> {
        let mut conn = self.db.acquire().await?;
        let mut tx = conn.begin().await?;
        let created_at: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let res = sqlx::query("INSERT INTO File(name, parent,
        is_deleted, created_at, modified_at, is_folder, is_bookmarked)
        VALUES (?, ?, FALSE, ?, ?, FALSE, FALSE)")
            .bind(name)
            .bind(parent_id)
            .bind(created_at)
            .bind(created_at)
            .execute(&mut *tx)
            .await?;

        let file: LocalFile = sqlx::query_as("SELECT * FROM File WHERE id=?")
            .bind(res.last_insert_rowid())
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(file)
    }

    /// List files under a parent.
    async fn list_files(&self, parent_id: Option<&str>) -> Result<Vec<LocalFile>> {
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

    /// Write a slice of bytes to a file.
    ///
    /// The file's content will be replaced with the bytes.
    async fn write_to_file(&self, _: &str, _: &[u8]) -> Result<LocalFile> {
        unimplemented!();
    }

    /// Read the file's binary data.
    async fn read_from_file(&self, _: &str) -> Result<Vec<u8>> {
        unimplemented!();
    }
}

impl Delta for Client {
    type File = LocalFile;
    type Error = Error;

    async fn list_deltas(&self, token: Option<&str>) -> Result<(Vec<LocalFile>, String)> {
        let mut conn = self.db.acquire().await?;
        let deltas: Vec<LocalFile> = match token {
            Some(token) => sqlx::query_as("SELECT * FROM File WHERE
        (modified_at >= synced_at OR synced_at IS NULL) AND
        modified_at >= ?")
                .bind(token)
                .fetch_all(&mut *conn)
                .await?,
            None => sqlx::query_as("SELECT * FROM File WHERE
        modified_at >= synced_at OR synced_at IS NULL")
                .fetch_all(&mut *conn)
                .await?
        };

        let current_time: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        Ok((deltas, format!("{}", current_time)))
    }
}

#[cfg(test)]
mod tests {

    use crate::local::schema;
    use super::*;
    use std::sync::Arc;
    use tokio::sync::OnceCell;
    static CLIENT: OnceCell<Arc<Client>> = OnceCell::const_new();

    async fn get_local_fs() -> Arc<Client> {
        CLIENT.get_or_init(|| async {
            let db_name = "./hs-localfs-test-db.sqlite";

            let _ = std::fs::remove_file(db_name);
            let _ = std::fs::remove_file(&format!("{db_name}-shm"));
            let _ = std::fs::remove_file(&format!("{db_name}-wal"));

            // Add files for testing.
            const TESTING_SCHEMA: &str = r#"
INSERT INTO File VALUES
  (0, "test.txt", NULL, NULL, FALSE, 0, 0, NULL, FALSE, FALSE),
  (1, "test-dltd", NULL, NULL, TRUE, 0, 0, NULL, TRUE, FALSE),
  (2, "my_folder", NULL, NULL, FALSE, 0, 0, NULL, TRUE, FALSE),
  (3, "move_me.txt", NULL, NULL, FALSE, 0, 0, NULL, FALSE, FALSE),
  (4, "delete_me.txt", NULL, NULL, FALSE, 0, 0, NULL, FALSE, FALSE),
  (5, "delete_me", NULL, NULL, FALSE, 0, 0, NULL, TRUE, FALSE),
  (6, "deleted_child.txt", 5, NULL, FALSE, 0, 0, NULL, FALSE, FALSE),
  (7, "test-list", NULL, NULL, FALSE, 0, 0, NULL, TRUE, FALSE),
  (8, "list-child.txt", 7, NULL, FALSE, 0, 0, NULL, FALSE, FALSE),
  (9, "test-list-deleted", NULL, NULL, FALSE, 0, 0, NULL, TRUE, FALSE),
  (10, "list-child.txt", 9, NULL, FALSE, 0, 0, NULL, FALSE, FALSE),
  (11, "bookmarked.txt", NULL, NULL, FALSE, 0, 0, NULL, FALSE, TRUE);

INSERT INTO Tag VALUES
  ("empty-tag", "ffffff", 0),
  ("tag1", "ff0000", 0),
  ("tag2", "00ff00", 0);

INSERT INTO TagBind VALUES
  ("tag1", 0),
  ("tag1", 2),
  ("tag2", 0);
"#;

            let db = database::Database::new(&database::Config {
                max_connections: 1,
                local_path: db_name.to_string(),
                migrations: vec![
                    database::Migration {
                        version: 0,
                        sql: schema::SCHEMA_VERSION_0.to_string(),
                        kind: database::MigrationType::Up,
                    },
                    database::Migration {
                        version: 1,
                        sql: TESTING_SCHEMA.to_string(),
                        kind: database::MigrationType::Up,
                    }
                ]
            }).await.unwrap();
            Arc::new(Client::new(Arc::new(db)))
        }).await.clone()
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
        let fs = get_local_fs().await;
        let ts: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        // Copy a file.
        let copied = fs.copy_file("0", Some("2"), Some(r#"copied.txt"#)).await.unwrap();
        assert_eq!(copied.name, r#"copied.txt"#);
        assert_eq!(copied.parent, Some(2));

        // Ensure timestamp is set.
        assert!(copied.created_at >= ts);
        assert!(copied.modified_at >= ts);

        // Ensure copied file appears within parent directory.
        let parent_files = fs.list_files(Some("2")).await.unwrap();
        assert!(parent_files.iter().any(|f| f.id == copied.id));

        // Copy a folder and its children.
        let copied_folder = fs.copy_file("7", Some("2"), Some(r#"copied_folder"#)).await.unwrap();
        assert_eq!(copied_folder.name, r#"copied_folder"#);
        assert_eq!(copied_folder.parent, Some(2));
        assert!(copied_folder.is_folder);
        assert!(copied_folder.created_at >= ts);
        assert!(copied_folder.modified_at >= ts);

        let children = fs.list_files(Some(&copied_folder.id.to_string())).await.unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name, r#"list-child.txt"#);

        // Do not allow copying deleted files.
        assert!(fs.copy_file("1", None, None).await.is_err());
    }

    #[tokio::test]
    async fn test_move_file() {
        let fs = get_local_fs().await;
        let ts: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let moved = fs.move_file("3", Some("2"), Some(r#"renamed.txt"#))
            .await.unwrap();

        assert!(moved.parent.is_some_and(|p| p == 2));
        assert!(moved.name == r#"renamed.txt"#);

        // Moving a file should update its `modified_at` field.
        assert!(moved.modified_at >= ts);

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
        let folder = fs.create_folder(None, r#"created-folder"#)
            .await.unwrap();

        assert!(folder.name == r#"created-folder"#);
        assert!(folder.parent.is_none());

        // Do not allow creating folders under deleted folders.
        assert!(fs.create_folder(Some("1"), r#"invalid-folder"#)
                .await.is_err());

        // Do not allow creating folders under files.
        assert!(fs.create_folder(Some("0"), r#"invalid-folder"#)
                .await.is_err());
    }

    #[tokio::test]
    async fn test_create_file() {
        let fs = get_local_fs().await;
        let file = fs.create_file(None, r#"created_file.txt"#)
            .await.unwrap();

        assert!(file.name == r#"created_file.txt"#);
        assert!(file.parent.is_none());

        // Do not allow creating files under deleted folders.
        assert!(fs.create_file(Some("1"), r#"invalid_file.txt"#)
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
    async fn test_list_deltas() {
        let fs = get_local_fs().await;
        let (_, token) = fs.list_deltas(None).await.unwrap();

        // Perform some change.
        let file = fs.create_file(None, "track-changes.txt")
            .await.unwrap();

        let (deltas, _) = fs.list_deltas(Some(&token))
            .await.unwrap();

        let mut has_change = false;
        deltas.iter().for_each(|item| {
            if item.name == file.name {
                has_change = true;
            }
        });

        assert!(has_change);
    }

    #[tokio::test]
    async fn test_write_to_file() {
    }

    #[tokio::test]
    async fn test_read_from_file() {
    }

    #[tokio::test]
    async fn test_create_bookmark() {
        let fs = get_local_fs().await;
        let file = fs.create_file(None, "bookmark_test_create_file.txt").await.unwrap();

        // Bookmark the file.
        assert!(fs.create_bookmark(&file.id.to_string()).await.is_ok());

        // Check if it's in the bookmarks list.
        let bookmarks = fs.list_bookmarks().await.unwrap();
        assert!(bookmarks.iter().any(|f| f.id == file.id));

        // Trying to bookmark it again should fail.
        assert!(fs.create_bookmark(&file.id.to_string()).await.is_err());

        // Do not allow bookmarking a deleted file.
        assert!(fs.create_bookmark("1").await.is_err());

        // Do not allow bookmarking a non-existent file.
        assert!(fs.create_bookmark("999").await.is_err());
    }

    #[tokio::test]
    async fn test_remove_bookmark() {
        let fs = get_local_fs().await;
        let file = fs.create_file(None, "bookmark_test_remove_file.txt").await.unwrap();
        fs.create_bookmark(&file.id.to_string()).await.unwrap();

        // Remove the bookmark.
        assert!(fs.remove_bookmark(&file.id.to_string()).await.is_ok());

        // Check it's gone from bookmarks.
        let bookmarks = fs.list_bookmarks().await.unwrap();
        assert!(!bookmarks.iter().any(|f| f.id == file.id));

        // Trying to remove it again should fail.
        assert!(fs.remove_bookmark(&file.id.to_string()).await.is_err());

        // Do not allow removing a bookmark from a file that is not bookmarked.
        assert!(fs.remove_bookmark("0").await.is_err());

        // Do not allow removing a bookmark from a non-existent file.
        assert!(fs.remove_bookmark("999").await.is_err());
    }

    #[tokio::test]
    async fn test_create_tag() {
        let fs = get_local_fs().await;
        let tag_name = "a-new-tag-for-testing-creation";
        let tag_color = "123456";

        // Create a new tag.
        let tag = fs.create_tag(tag_name, tag_color).await.unwrap();
        assert_eq!(tag.name, tag_name);
        assert_eq!(tag.color, tag_color);

        // Verify it's in the list of all tags.
        let tags = fs.list_tags().await.unwrap();
        let found_tag = tags.iter().find(|t| t.name == tag_name).unwrap();
        assert_eq!(found_tag.name, tag_name);
        assert_eq!(found_tag.color, tag_color);

        // Creating a tag that already exists should fail.
        assert!(fs.create_tag(tag_name, "000000").await.is_err());
    }

    #[tokio::test]
    async fn test_create_tag_bind() {
        let fs = get_local_fs().await;
        let tag_name = "tag-for-binding-test";
        let file_name = "file-for-tag-binding.txt";

        let file = fs.create_file(None, file_name).await.unwrap();
        fs.create_tag(tag_name, "ff00ff").await.unwrap();

        // Bind tag to file.
        assert!(fs.create_tag_bind(&file.id.to_string(), tag_name).await.is_ok());

        // Verify the bind.
        let tags = fs.list_tags().await.unwrap();
        let tag = tags.iter().find(|t| t.name == tag_name).unwrap();
        assert_eq!(tag.files.len(), 1);
        assert_eq!(tag.files[0].id, file.id);

        // Do not allow binding non-existent tag.
        assert!(fs.create_tag_bind(&file.id.to_string(), "no-such-tag").await.is_err());

        // Do not allow binding to non-existent file.
        assert!(fs.create_tag_bind("999", tag_name).await.is_err());

        // Do not allow binding to a deleted file.
        assert!(fs.create_tag_bind("1", tag_name).await.is_err());

        // Do not allow duplicate bindings.
        assert!(fs.create_tag_bind(&file.id.to_string(), tag_name).await.is_err());
    }

    #[tokio::test]
    async fn test_remove_tag_bind() {
        let fs = get_local_fs().await;
        let tag_name = "tag-for-removal-test";
        let file1_name = "file1-for-tag-removal.txt";
        let file2_name = "file2-for-tag-removal.txt";

        fs.create_tag(tag_name, "ff00ff").await.unwrap();
        let file1 = fs.create_file(None, file1_name).await.unwrap();
        let file2 = fs.create_file(None, file2_name).await.unwrap();
        fs.create_tag_bind(&file1.id.to_string(), tag_name).await.unwrap();
        fs.create_tag_bind(&file2.id.to_string(), tag_name).await.unwrap();

        // Remove one bind.
        assert!(fs.remove_tag_bind(&file1.id.to_string(), tag_name).await.is_ok());

        // Verify that the tag still exists and is bound to file2.
        let tags = fs.list_tags().await.unwrap();
        let tag = tags.iter().find(|t| t.name == tag_name).unwrap();
        assert_eq!(tag.files.len(), 1);
        assert_eq!(tag.files[0].id, file2.id);

        // Removing a non-existent bind should succeed but do nothing.
        assert!(fs.remove_tag_bind(&file1.id.to_string(), tag_name).await.is_ok());
        assert!(fs.remove_tag_bind("0", "no-such-tag").await.is_ok());
        assert!(fs.remove_tag_bind("999", "tag1").await.is_ok());
    }
}
