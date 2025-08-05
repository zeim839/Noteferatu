use serde::{Serialize, Deserialize};

/// Describes a model available via the API.
///
/// API Reference: [Models](https://openrouter.ai/docs/api-reference/list-available-models)
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

impl Into<crate::core::Model> for Model {
    fn into(self) -> crate::core::Model {
        crate::core::Model {
            id: self.id.clone(),
            display_name: self.id.clone(),
            provider: "OpenRouter".to_string(),
            context_size: 200000,
        }
    }
}
