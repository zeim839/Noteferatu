use crate::config::*;
use std::path::Path;

use sqlx::migrate::MigrateDatabase;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::Sqlite;
use sqlx::Row;

/// Database result alias.
pub type Result<T> = std::result::Result<T, sqlx::Error>;

/// Database handles an SQLite connection pool.
pub struct Database(sqlx::pool::Pool<Sqlite>);

impl Database {

    /// Initializes a database using the given [Config].
    ///
    /// Creates the database file at [Config::local_path] if it
    /// doesn't already exist. Applies any migration whose version
    /// is greater than the value returned by [Database::version].
    ///
    /// Migrations will be applied in order of increasing version
    /// number. The `new` function will never downgrade the schema
    /// version.
    ///
    /// # Panics
    /// Panics if [Config::local_path] is invalid.
    pub async fn new(config: &Config) -> Result<Self> {
        let path = Path::new(&config.local_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .expect(&format!("invalid path: {}", config.local_path));
        }

        let path = path.to_str().unwrap();
        if !Sqlite::database_exists(path).await.unwrap_or(false) {
            Sqlite::create_database(path).await?;
        }

        let db = Self(
            sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(config.max_connections)
                .connect(&format!("sqlite://{}", path))
                .await?
        );

        // Initialize migrations table.
        let mut conn = db.acquire().await?;
        sqlx::query(MIGRATION_SCHEMA).execute(&mut *conn).await?;
        conn.close().await?;

        // Apply migrations.
        db.apply_migrations(&config.migrations).await?;

        Ok(db)
    }

    /// Acquire an SQLite database connection from the pool.
    pub async fn acquire(&self) -> Result<PoolConnection<Sqlite>> {
        let conn = self.0.acquire().await?;
        Ok(conn)
    }

    /// Get the latest applied [Migration] version.
    ///
    /// If output is `None`, then no migrations have been applied.
    pub async fn version(&self) -> Result<Option<i64>> {
        let res = sqlx::query("SELECT MAX(version) AS version FROM _migrations")
            .fetch_one(&self.0).await?;

        Ok(res.get("version"))
    }

    /// Apply the given migrations and then return the current schema
    /// version.
    pub(crate) async fn apply_migrations(&self, migrations: &[Migration]) -> Result<Option<i64>> {
        let mut migrations: Vec<Migration> = migrations.to_vec();
        migrations.sort_by_cached_key(|m| m.version);
        let current_version = self.version().await?;
        for migration in migrations {

            // Skip applied migrations.
            if let Some(current_version) = current_version {
                if migration.version <= current_version {
                    continue;
                }
            }
            if migration.kind != MigrationType::Up {
                continue;
            }

            // Apply migration.
            let mut conn = self.acquire().await?;
            sqlx::query(migration.sql).execute(&mut *conn).await?;

            // Report success.
            sqlx::query("INSERT INTO _migrations VALUES (?,?)")
                .bind(migration.version)
                .bind(migration.sql)
                .execute(&mut *conn)
                .await?;
        }

        self.version().await
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_execute_query() {

    }

    #[tokio::test]
    async fn test_apply_migrations() {
    }
}
