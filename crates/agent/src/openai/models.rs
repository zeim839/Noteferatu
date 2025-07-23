use serde::{Serialize, Deserialize};

/// Describes a model available via the API.
///
/// API Reference: [Models](https://platform.openai.com/docs/api-reference/models)
#[derive(Serialize, Deserialize)]
pub struct Model {

    /// Unique model identifier.
    pub id: String,

    /// Timestamp of creation date.
    pub created: i64,

    /// Model owner.
    ///
    /// Differentiates between models created by `openai` and custom
    /// fine-tuned models.
    pub owned_by: Option<String>,
}

impl Into<crate::Model> for Model {
    fn into(self) -> crate::Model {
        crate::Model {
            id: self.id.clone(),
            display_name: self.id.clone(),
            provider: "OpenAI".to_string(),
            context_size: 200000,
        }
    }
}
