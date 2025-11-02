/// Result alias.
pub type Result<T> = std::result::Result<T, Error>;

/// Error implementation.
#[derive(Debug, thiserror::Error)]
pub enum Error {
}
