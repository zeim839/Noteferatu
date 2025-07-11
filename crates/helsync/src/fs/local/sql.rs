use crate::database::{Database, Migration, MigrationList};
use sqlx::migrate::MigrationType;
use anyhow::Result;

const SQL_SCHEMA_V0: &str = "
CREATE TABLE IF NOT EXISTS File(
  id          INTEGER       PRIMARY KEY AUTO INCREMENT,
  name        TEXT          NOT NULL,
  parent      INTEGER,
  remote_id   VARCHAR(100),
  is_deleted  BOOLEAN       NOT NULL,
  created_at  INTEGER       NOT NULL,
  modified_at INTEGER       NOT NULL,
  synced_at   INTEGER,
  is_folder   BOOLEAN       NOT NULL,
  FOREIGN KEY (parent) REFERENCES File(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
);
";

const SQL_MIGRATION_V0: Migration = Migration {
    version: 0,
    description: "Initializes database",
    sql: SQL_SCHEMA_V0,
    kind: MigrationType::ReversibleUp,
};

/// Incrementally applies a list of migrations to update the database
/// schema to the latest version.
pub(crate) async fn apply_migrations(db: &Database) -> Result<()> {
    db.apply_migrations(MigrationList::new(vec![SQL_MIGRATION_V0])).await
}
