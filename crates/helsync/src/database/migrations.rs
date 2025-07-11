use futures_core::future::BoxFuture;
use sqlx::migrate::{MigrationType, Migration as SqlxMigration};
use sqlx::migrate::MigrationSource;
use sqlx::error::BoxDynError;
use std::result::Result;

/// A migration definition.
#[derive(Debug)]
pub struct Migration {
    pub version: i64,
    pub description: &'static str,
    pub sql: &'static str,
    pub kind: MigrationType,
}

/// A vector of migrations that implements
/// [MigrationSource](sqlx::migrate::MigrationSource).
#[derive(Debug)]
pub struct MigrationList(Vec<Migration>);

impl MigrationList {

    /// Create a MigrationList from a [Migration] vector.
    pub fn new(migrations: Vec<Migration>) -> Self {
        Self(migrations)
    }
}

impl MigrationSource<'static> for MigrationList {
    fn resolve(self) -> BoxFuture<'static, Result<Vec<SqlxMigration>, BoxDynError>> {
        Box::pin(async move {
            let mut migrations = Vec::new();
            for migration in self.0 {
                if matches!(migration.kind, MigrationType::ReversibleUp) {
                    migrations.push(SqlxMigration::new(
                        migration.version,
                        migration.description.into(),
                        migration.kind.into(),
                        migration.sql.into(),
                        false,
                    ));
                }
            }
            Ok(migrations)
        })
    }
}
