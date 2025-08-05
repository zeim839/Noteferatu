use serde::{Serialize, Deserialize};

/// A Google AI model definition.
///
/// API Reference: [Model](https://ai.google.dev/api/models#Model)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {

    /// The resource name of the Model.
    pub name: String,

    /// The version number of the model.
    pub version: String,

    /// The human-readable name of the model.
    ///
    /// E.g. "Gemini 1.5 Flash".
    /// The name can be up to 128 characters long and can consist of
    /// any UTF-8 characters.
    pub display_name: String,

    /// Maximum number of input tokens allowed for this model.
    pub input_token_limit: i64,

    /// Maximum number of output tokens available for this model.
    pub output_token_limit: i64,

    /// The model's supported generation methods.
    pub supported_generation_methods: Vec<String>,

    /// Whether the model supports thinking.
    pub thinking: Option<bool>,

    /// Controls the randomness of the output.
    pub temperature: Option<f64>,

    /// The maximum temperature this model can use.
    pub max_temperature: Option<f64>,

    /// Nucleus sampling.
    ///
    /// Nucleus sampling considers the smallest set of tokens whose
    /// probability sum is at least `top_p`. This value specifies default
    /// to be used by the backend while making the call to the model.
    pub top_p: Option<f64>,

    /// For Top-k sampling.
    ///
    /// Top-k sampling considers the set of topK most probable
    /// tokens. This value specifies default to be used by the backend
    /// while making the call to the model. If empty, indicates the model
    /// doesn't use top-k sampling, and topK isn't allowed as a generation
    /// parameter.
    pub top_k: Option<f64>,
}

impl Into<crate::core::Model> for Model {
    fn into(self) -> crate::core::Model {
        crate::core::Model {
            id: self.name[7..].to_string(),
            provider: "Google".to_string(),
            display_name: self.display_name,
            context_size: self.input_token_limit as u64,
        }
    }
}
