use serde::{Serialize, Deserialize};

/// A model definition.
#[derive(Serialize, Deserialize)]
pub struct Model {

    /// A user-friendly name of the model.
    pub name: String,

    /// The proper model name.
    pub model: String,

    /// The model's parameter size.
    pub size: i64,

    /// Whether tool calling is supported.
    pub supports_tool_calls: Option<bool>,

    /// Model context length.
    pub context_length: Option<u64>,
}

impl Into<crate::Model> for Model {
    fn into(self) -> crate::Model {
        crate::Model {
            id: self.name.clone(),
            display_name: self.name.clone(),
            provider: "Ollama".to_string(),
            context_size: self.context_length.unwrap_or(20000),
        }
    }
}
