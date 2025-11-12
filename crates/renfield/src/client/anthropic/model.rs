//! Model information.

use serde::Deserialize;

/// Model information.
///
/// See: [API Reference](https://docs.claude.com/en/api/models-list)
#[derive(Deserialize, Debug, Clone)]
pub struct Model {

    /// RFC 3339 datetime string representing the time at which the
    /// model was released.
    ///
    /// May be set to an epoch value if the release date is unknown.
    ///
    /// Examples: `2025-02-19T00:00:00Z`.
    pub created_at: String,

    /// A human-readable name for the model.
    ///
    /// Examples: `Claude Sonnet 4`.
    pub display_name: String,

    /// Unique model identifier.
    ///
    /// Examples: `claude-sonnet-4-20250514`.
    pub id: String,
}
