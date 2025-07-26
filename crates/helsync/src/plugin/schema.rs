use database::{Migration, MigrationType};

pub(crate) const MIGRATION_V0: Migration = Migration {
    version: 0,
    sql: crate::local::SCHEMA_VERSION_0,
    kind: MigrationType::Up,
};
