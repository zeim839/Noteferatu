use anyhow::Result;
use std::path::Path;

use sqlx::migrate::{Migrator, MigrateDatabase, MigrationSource};
use sqlx::pool::PoolConnection;
use sqlx::sqlite::Sqlite;
use sqlx::Row;

/// Database handles an SQLite connection pool.
pub struct Database(sqlx::pool::Pool<Sqlite>);

impl Database {

    /// Initializes an SQLite database at the specified path.
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_str().unwrap();
        if !Sqlite::database_exists(path_str).await.unwrap_or(false) {
            Sqlite::create_database(path_str).await?;
        }

        // Create database pool.
        let conn_str = format!("sqlite://{}", path_str);
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&conn_str)
            .await?;

        Ok(Self(pool))
    }

    /// Get the latest applied migrations version.
    pub async fn version(&self) -> Result<i64> {
        let res = sqlx::query("SELECT MAX(version) AS version FROM _sqlx_migrations WHERE success=TRUE")
            .fetch_one(&self.0).await?;

        Ok(res.get("version"))
    }

    /// Applies a [MigrationList](super::migrations::MigrationList).
    pub async fn apply_migrations<'s, S: MigrationSource<'s>>(&self, source: S) -> Result<()> {
        let mut migrator = Migrator::new(source).await?;
        migrator.set_locking(true).run(&self.0).await?;
        Ok(())
    }

    /// Acquire an SQLite database connection from the pool.
    pub async fn acquire(&self) -> Result<PoolConnection<Sqlite>> {
        self.0.acquire().await
            .map_err(|_| anyhow::anyhow!("could not connect to db"))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::*;
    use sqlx::migrate::MigrationType;

    #[tokio::test]
    async fn test_apply_migrations() {
        let sql = "CREATE TABLE Test(id INTEGER PRIMARY KEY);";
        let migration = Migration {
            version: 420,
            description: "my test migration",
            sql,
            kind: MigrationType::ReversibleUp,
        };

        let db = Database::new("./test.sqlite").await.unwrap();
        db.apply_migrations(MigrationList::new(vec![migration]))
            .await.unwrap();

        let version = db.version().await.unwrap();
        assert!(version == 420);

        let _ = std::fs::remove_file("./test.sqlite");
        let _ = std::fs::remove_file("./test.sqlite-shm");
        let _ = std::fs::remove_file("./test.sqlite-wal");
    }
}
