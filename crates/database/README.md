# database

The `database` crate is a simple SQLite wrapper over [sqlx](https://github.com/launchbadge/sqlx). It manages its own migrations table and doesn't rely on custom SQLx migration files.

## Examples
## Initialize database and execute a query.
```rust
use database::*;

async fn run() {

    // Configure database.
    let config = Config {
        max_connections: 5,
        local_path: "/path/to/db.sqlite".to_string(),
        migrations: vec![],
    };

    // Initialize database.
    let db = Database::new(&config).await.unwrap();

    // Acquire a database connection.
    let mut conn = db.acquire().await.unwrap();

    // Execute a query.
    sqlx::query("INSERT INTO Table VALUES()")
        .execute(&mut *conn)
        .await.unwrap();
}
```

## Migrations
Migrations allow you to upgrade database schemas on already-deployed database files.
```rust
use database::*;

async fn run() {

    // Migrations.
    const MIGRATION_V0: Migration = Migration {
        version: 0,
        sql: "CREATE TABLE IF NOT EXISTS MyTable (id INTEGER PRIMARY KEY)",
        kind: MigrationType::Up,
    };

    const MIGRATION_V1: Migration = Migration {
        version: 1,
        sql: "DROP TABLE MyTable",
        kind: MigrationType::Up,
    };

    // Configure database.
    let config = Config {
        max_connections: 5,
        local_path: "/path/to/db.sqlite".to_string(),
        migrations: vec![MIGRATION_V0, MIGRATION_V1],
    };

    // Initialize database.
    let db = Database::new(&config).await.unwrap();

    // Get current schema version.
    let version = db.version().await.unwrap();
    println!("{version}");
}
```
