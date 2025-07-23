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

impl Into<crate::Model> for Model {
    fn into(self) -> crate::Model {
        crate::Model {
            id: self.id,
            display_name: self.display_name,
            provider: "Anthropic".to_string(),
            context_size: 200000,
        }
    }
}
