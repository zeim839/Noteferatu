/// Migration variants.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MigrationType {
    Up, Down
}

/// A migration definition.
///
/// Migrations allow you to upgrade [Database](crate::Database)
/// schemas after a database has already been deployed.
///
/// When calling [Database::new](crate::Database::new), the database
/// object will try to apply any migration whose version is greater
/// than its current version (i.e. the value returned by
/// [Database::version](crate::Database::version)).
#[derive(Clone, Copy, Debug)]
pub struct Migration {

    /// The schema version after applying the migration.
    pub version: i64,

    /// The SQL statement to execute.
    pub sql: &'static str,

    /// Whether this migration upgrades or downgrades the current
    /// schema version.
    ///
    /// Calling [Database::new](crate::Database::new) will never
    /// downgrade a database schema.
    pub kind: MigrationType,
}

/// The migrations table schema.
pub(crate) const MIGRATION_SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS _migrations (
  version INTEGER PRIMARY KEY,
  sql     TEXT    NOT NULL
);
";

/// Database configuration.
///
/// The order of the [Migration::kind] field elements does not
/// matter. Migrations will be applied in order of increasing version
/// number.
pub struct Config {

    /// Maximum number of concurrent database connections.
    pub max_connections: u32,

    /// Path to the SQLite database file.
    ///
    /// E.g. `~/Users/MyUser/Desktop/db.sqlite`.
    pub local_path: String,

    /// List of schema migrations to apply.
    pub migrations: Vec<Migration>,
}
