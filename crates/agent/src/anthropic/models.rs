use serde::{Serialize, Deserialize};

/// Model metadata.
#[derive(Serialize, Deserialize)]
pub struct Model {

    /// RFC 3339 datetime string representing the time at which the
    /// model was released. May be set to an epoch value if the
    /// release date is unknown.
    pub created_at: String,

    /// A human-readable name for the model.
    pub display_name: String,

    /// Unique model identifier.
    pub id: String,
}
