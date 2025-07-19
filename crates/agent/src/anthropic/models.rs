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

impl crate::ModelDefinition for Model {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn display_name(&self) -> String {
        self.display_name.clone()
    }

    fn context_length(&self) -> u64 {
        200000
    }

    fn supports_tool_calls(&self) -> bool {
        true
    }

    fn supports_web_search(&self) -> bool {
        true
    }
}
