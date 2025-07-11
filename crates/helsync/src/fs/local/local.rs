use super::file::LocalFile;

use crate::database::Database;
use crate::fs::utils::handle_not_found_err;
use crate::fs::FS;

use std::path::Path;
use anyhow::{anyhow, Result};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct LocalFS<F: FS> {
    db: Database,
    tk: Option<String>,
    re: Option<F>,
}

impl<F: FS> LocalFS<F> {
    pub async fn new<P: AsRef<Path>>(local: P, remote: Option<F>) -> Result<Self> {
        let path_str = local.as_ref().to_str()
            .ok_or(anyhow!("could not read LocalFS path"))?;

        // Create parent directory.
        std::fs::create_dir_all(&local)
            .map_err(|_| anyhow!("could not create path \"{}\"", path_str))?;

        // Initialize database.
        let db_path = local.as_ref().join("db.sqlite");
        let db = Database::new(db_path).await?;
        super::sql::apply_migrations(&db).await?;

        Ok(Self{db, tk: None, re: remote})
    }
}

impl<F: FS> FS for LocalFS<F> {
    type File = LocalFile;
    type Delta = LocalFile;

    async fn get_file(&self, id: &str) -> Result<Self::File> {
        let mut conn = self.db.acquire().await?;
        let file: LocalFile = sqlx::query_as("SELECT * FROM File WHERE id=?")
            .bind(id)
            .fetch_one(&mut *conn).await
            .map_err(handle_not_found_err)?;

        if file.is_deleted {
            return Err(anyhow!("file has been deleted"));
        }

        Ok(file)
    }

    async fn copy_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) -> Result<Self::File> {
        let file = self.get_file(source_id).await?;
        let new_name = match name {
            Some(name) => name.to_string(),
            None => file.name,
        };

        let mut conn = self.db.acquire().await?;
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as i64;

        let res = sqlx::query("INSERT INTO File (name, parent,
        is_deleted, created_at, modified_at, is_folder) VALUES (?, ?,
        ?, ?, ?, ?);")
            .bind(&new_name)
            .bind(parent_id)
            .bind(false)
            .bind(ts)
            .bind(ts)
            .bind(file.is_folder)
            .execute(&mut *conn).await?;

        let new_file = LocalFile {
            id: res.last_insert_rowid(),
            name: new_name,
            parent: parent_id.map(|id| id.parse::<i64>().unwrap()),
            remote_id: None,
            is_deleted: false,
            created_at: ts,
            modified_at: ts,
            synced_at: None,
            is_folder: file.is_folder,
        };

        Ok(new_file)
    }

    async fn move_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) -> Result<Self::File> {
        let mut conn = self.db.acquire().await?;
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as i64;

        let res = match name {
            Some(name) => sqlx::query(
                "UPDATE File SET name=?, parent_id=?, modified_at=?
            WHERE id=? AND is_deleted=FALSE"
            )
                .bind(name)
                .bind(parent_id)
                .bind(ts)
                .bind(source_id)
                .execute(&mut *conn)
                .await?,
            None => sqlx::query(
                "UPDATE File SET parent_id=?, modified_at=? WHERE id=?
            AND is_deleted=FALSE"
            )
                .bind(parent_id)
                .bind(ts)
                .bind(source_id)
                .execute(&mut *conn)
                .await?
        };

        if res.rows_affected() == 0 {
            return Err(anyhow!("file is deleted or could not be found"));
        }

        Ok(self.get_file(source_id).await?)
    }

    async fn remove_file(&self, id: &str) -> Result<()> {
        let mut conn = self.db.acquire().await?;
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as i64;

        let res = sqlx::query("UPDATE File SET is_deleted=TRUE,
        modified_at=? WHERE id=? AND is_deleted=FALSE; UPDATE File SET
        is_deleted=TRUE, modified_at=? WHERE parent_id=?")
            .bind(ts)
            .bind(id)
            .bind(ts)
            .bind(id)
            .execute(&mut *conn)
            .await?;

        if res.rows_affected() == 0 {
            return Err(anyhow!("file is deleted or could not be found"));
        }

        Ok(())
    }

    async fn create_folder(&self, parent_id: Option<&str>, name: &str) -> Result<Self::File> {
        let mut conn = self.db.acquire().await?;
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as i64;

        let res = sqlx::query("INSERT INTO File(name, parent,
        is_deleted, created_at, modified_at, is_folder) VALUES (?, ?,
        ?, ?, ?, ?)")
            .bind(name)
            .bind(parent_id)
            .bind(false)
            .bind(ts)
            .bind(ts)
            .bind(true)
            .execute(&mut *conn)
            .await?;

        self.get_file(&res.last_insert_rowid().to_string()).await
    }

    async fn list_files(&self, parent_id: Option<&str>) -> Result<Vec<Self::File>> {
        let mut conn = self.db.acquire().await?;
        let rows: Vec<LocalFile> = sqlx::query_as("SELECT * FROM File WHERE parent_id=? AND is_deleted=FALSE")
            .bind(parent_id)
            .fetch_all(&mut *conn)
            .await?;

        Ok(rows)
    }

    async fn track_changes(&self, parent_id: Option<&str>, token: Option<&str>) -> Result<(Vec<Self::Delta>, String)> {
        unimplemented!();
    }

    async fn write_to_file(&self, buf: &[u8], parent_id: Option<&str>, name: &str) -> Result<Self::File> {
        unimplemented!();
    }

    async fn read_from_file(&self, id: &str) -> Result<Vec<u8>> {
        unimplemented!();
    }
}
