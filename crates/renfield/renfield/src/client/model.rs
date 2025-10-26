/// Large language model specification.
///
/// Defines metadata and capabilities of a model provided by a
/// [Client](super::Client).
pub struct Model {

    /// A unique name or identifier for the model.
    pub id: String,

    /// Input types supported by the model.
    pub input_modalities: Vec<Modality>,

    /// Output types supported by the model.
    pub output_modalities: Vec<Modality>,

    /// The model's context window.
    pub context_size: u64,

    /// Whether the model supports tool calling.
    pub supports_tools: bool,

    /// Whether the model supports internal reasoning.
    pub supports_reasoning: bool,
}

/// Input/output types supported by a [Model].
pub enum Modality {
    Text,
    Image,
    Video,
    Audio,
}
