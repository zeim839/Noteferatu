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

impl crate::ModelDefinition for Model {
    fn id(&self) -> String {
        self.model.clone()
    }

    fn display_name(&self) -> String {
        self.name.clone()
    }

    fn context_length(&self) -> u64 {
        self.context_length.unwrap_or(20000)
    }

    fn supports_tool_calls(&self) -> bool {
        self.supports_tool_calls.unwrap_or(false)
    }

    fn supports_web_search(&self) -> bool {
        false
    }
}
