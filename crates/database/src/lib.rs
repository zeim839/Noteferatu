//! SQLite Database Wrapper.
//!
//! # Examples
//! ## Initialize database and execute a query.
//! ```no_run
//! use database::*;
//!
//! async fn run() {
//!
//!     // Configure database.
//!     let config = Config {
//!         max_connections: 5,
//!         local_path: "/path/to/db.sqlite".to_string(),
//!         migrations: vec![],
//!     };
//!
//!     // Initialize database.
//!     let db = Database::new(&config).await.unwrap();
//!
//!     // Acquire a database connection.
//!     let mut conn = db.acquire().await.unwrap();
//!
//!     // Execute a query.
//!     sqlx::query("INSERT INTO Table VALUES()")
//!         .execute(&mut *conn)
//!         .await.unwrap();
//! }
//! ```
//!
//! ## Migrations
//! Migrations allow you to upgrade database schemas on
//! already-deployed database files.
//! ```no_run
//! use database::*;
//!
//! async fn run() {
//!
//!     // Migrations.
//!     const MIGRATION_V0: Migration = Migration {
//!         version: 0,
//!         sql: "CREATE TABLE IF NOT EXISTS MyTable (id INTEGER PRIMARY KEY)",
//!         kind: MigrationType::Up,
//!     };
//!
//!     const MIGRATION_V1: Migration = Migration {
//!         version: 1,
//!         sql: "DROP TABLE MyTable",
//!         kind: MigrationType::Up,
//!     };
//!
//!     // Configure database.
//!     let config = Config {
//!         max_connections: 5,
//!         local_path: "/path/to/db.sqlite".to_string(),
//!         migrations: vec![MIGRATION_V0, MIGRATION_V1],
//!     };
//!
//!     // Initialize database.
//!     let db = Database::new(&config).await.unwrap();
//!
//!     // Get current schema version.
//!     let version = db.version().await.unwrap();
//!     println!("{version}");
//! }
//! ```
mod database;
mod config;

pub use database::*;
pub use config::*;
