use serde::{Serialize, Deserialize};

/// Implements a generic LLM definition that captures basic attributes
/// from all clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub id: String,
    pub display_name: String,
    pub provider: String,
    pub context_size: u64,
}
