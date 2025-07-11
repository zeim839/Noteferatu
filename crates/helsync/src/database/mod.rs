//! SQLite database wrapper.
//!
//! # Examples
//! ## Create a new SQLite Database
//! ```no_run
//! use helsync::database::*;
//!
//! #[tokio::main]
//! async fn main() {
//!
//!     // Connect to the database at the path, creating it if it
//!     // doesn't exist.
//!     let db = Database::new("/path/to/db.sqlite")
//!         .await.unwrap();
//!
//!     // Acquire a connection and execute a query.
//!     let mut conn = db.acquire().await.unwrap();
//!     sqlx::query("INSERT INTO Table VALUES()")
//!         .execute(&mut *conn)
//!         .await.unwrap();
//!
//! }
//! ```
//!
//! ## Migrate schema to a newer version
//! ```no_run
//! use helsync::database::*;
//! use sqlx::migrate::MigrationType;
//!
//! #[tokio::main]
//! async fn main() {
//!
//!     // Define a migration.
//!     let sql = "CREATE TABLE Test(id INTEGER PRIMARY KEY);";
//!     let migration = Migration {
//!         version: 1,
//!         description: "my migration description",
//!         sql,
//!         kind: MigrationType::ReversibleUp,
//!     };
//!
//!     let db = Database::new("/path/to/db.sqlite")
//!         .await.unwrap();
//!
//!     // Apply the migration.
//!     db.apply_migrations(MigrationList::new(vec![migration]))
//!         .await.unwrap();
//!
//!     // Check latest version.
//!     let version = db.version().await.unwrap();
//!     assert!(version == 1);
//! }
//! ```

mod database;
pub use database::*;

mod migrations;
pub use migrations::*;
