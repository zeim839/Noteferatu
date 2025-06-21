use anyhow::Result;
use dirs::home_dir;

use sqlx::migrate::MigrateDatabase;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::Sqlite;

use clap::ValueEnum;

/// Database handles an SQLite connection pool.
pub struct Database(sqlx::pool::Pool<Sqlite>);

impl Database {

    /// Initializes a new database controller. It creates a ".helsync"
    /// folder in the user's root directory and a ".helsync/db.sqlite"
    /// database file (if they do not already exist).
    pub async fn new() -> Result<Self> {

        // Create root config directory.
        let config_dir = home_dir()
            .expect("could not find user home directory")
            .join(".helsync");

        std::fs::create_dir_all(&config_dir)?;

        // Create drives directory.
        std::fs::create_dir_all(config_dir.join("drives"))?;

        // Create SQLite database file.
        let db_path = String::from(config_dir.join("db.sqlite").to_str().unwrap());
        if !Sqlite::database_exists(&db_path).await.unwrap_or(false) {
            Sqlite::create_database(&db_path).await?;
        }

        // Create database pool.
        let conn_str = format!("sqlite://{}", db_path);
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&conn_str)
            .await?;

        // Create tables.
        sqlx::query(SQL_SCHEMA).execute(&pool).await
            .map_err(|_| anyhow::anyhow!("could not initialize db tables"))?;

        Ok(Self(pool))
    }

    /// Acquire an SQLite database connection from the pool.
    pub async fn acquire(&self) -> Result<PoolConnection<Sqlite>> {
        self.0.acquire().await
            .map_err(|_| anyhow::anyhow!("could not connect to db"))
    }
}

/// BindKind enumerates the possible types of a drive binding,
/// i.e. either local or remote.
#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum BindKind {
    Local,
    Remote,
}

impl std::fmt::Display for BindKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BindKind::Local => write!(f, "local"),
            BindKind::Remote => write!(f, "remote"),
        }
    }
}

/// CloudProvider enumerates the supported cloud APIs.
#[derive(Debug, Clone, ValueEnum, sqlx::Type)]
pub enum CloudProvider {
    #[value(name = "GoogleDrive")]
    GoogleDrive,
    #[value(name = "OneDrive")]
    OneDrive,
}

impl std::fmt::Display for CloudProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudProvider::GoogleDrive => write!(f, "GoogleDrive"),
            CloudProvider::OneDrive => write!(f, "OneDrive"),
        }
    }
}

/// The CLI uses an SQLite database to track its state that is
/// separate from the database that a drive from the library would
/// use. Drive databases are stored within the CLI's configuration
/// folder, i.e. $HOME/.helsync.
const SQL_SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS App (
  name          VARCHAR(20) PRIMARY KEY,
  provider      VARCHAR(20) NOT NULL,
  client_id     TEXT        NOT NULL,
  port          INTEGER     NOT NULL,
  client_secret TEXT
);

CREATE TABLE IF NOT EXISTS Drive (
  name VARCHAR(20) PRIMARY KEY,
  path TEXT NOT NULL,
  app  VARCHAR(20) NOT NULL,

  access_token  TEXT,
  refresh_token TEXT,
  created_at    INTEGER,
  expires_in    INTEGER,

  FOREIGN KEY (app) REFERENCES App(name)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS Filesystem (
  path  TEXT        PRIMARY KEY,
  drive VARCHAR(20) NOT NULL UNIQUE,

  FOREIGN KEY (drive) REFERENCES Drive(name)
    ON UPDATE CASCADE
    ON DELETE CASCADE
);
";
