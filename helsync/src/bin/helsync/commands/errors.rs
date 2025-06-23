use sqlx::Error as SqlxError;
use sqlx::error::DatabaseError;
use sqlx::sqlite::SqliteError;

/// Checks for the possibility of a RowNotFound SQL error and returns
/// a user-friendly error message.
pub(crate) fn handle_not_found_err(e: SqlxError) -> anyhow::Error {
    match e {
        SqlxError::RowNotFound => {
            anyhow::anyhow!("object not found")
        },
        _ => anyhow::anyhow!("{e}")
    }
}

/// Checks for the possibility of a UNIQUE violation constraint SQL
/// error and returns a user-friendly error message.
pub(crate) fn handle_unique_violation_err(e: SqlxError) -> anyhow::Error {
    match e {
        SqlxError::Database(db_err) => {
            let sqlite_err = db_err.downcast_ref::<SqliteError>();
            if sqlite_err.code().unwrap_or_default() == "2067" ||
                sqlite_err.code().unwrap_or_default() == "1555" {
                    return anyhow::anyhow!("object already exists");
                }
            return anyhow::anyhow!("database error");
        },
        _ => anyhow::anyhow!("{e}"),
    }
}
