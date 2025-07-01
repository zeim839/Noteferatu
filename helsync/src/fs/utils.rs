use std::collections::HashMap;
use sqlx::Error as SqlxError;

/// Extract query parameters from a URL string
pub(crate) fn extract_query_params(url: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    if let Some(query_start) = url.find('?') {
        let query = &url[query_start + 1..];
        for pair in query.split('&') {
            if let Some(eq_pos) = pair.find('=') {
                let key = pair[..eq_pos].to_string();
                let value = pair[eq_pos + 1..].to_string();
                params.insert(key, value);
            }
        }
    }
    params
}

pub(crate) fn handle_read_lock_err<T: std::error::Error>(_: T) -> anyhow::Error {
    anyhow::anyhow!("Failed to acquire read lock on client state")
}

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
