//! Model information.

use serde::Deserialize;

/// Gemini model description.
///
/// Information about a Generative Language Model.
///
/// See: [API Reference](https://ai.google.dev/api/models#Model)
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Model {

    /// The resource name of the [Model].
    ///
    /// Format:
    ///  * models/{model} with a {model} naming convention of:
    ///
    /// Example: `models/gemini-1.5-flash-001`
    pub name: String,

    /// The name of the base model, pass this to the generation
    /// request.
    ///
    /// Example: `gemini-1.5-flash`.
    pub base_model_id: Option<String>,

    /// The version number of the model.
    pub version: String,

    /// The human-readable name of the model. E.g. "Gemini 1.5 Flash".
    pub display_name: String,

    /// A short description of the model.
    pub description: Option<String>,

    /// Maximum number of input tokens allowed for this model.
    pub input_token_limit: u64,

    /// Maximum number of output tokens available for this model.
    pub output_token_limit: u64,

    /// The model's supported generation methods.
    pub supported_generation_methods: Vec<String>,

    /// Whether the model supports thinking.
    #[serde(default)]
    pub thinking: bool,

    /// Controls the randomness of the output.
    pub temperature: Option<f64>,

    /// The maximum temperature this model can use.
    pub max_temperature: Option<f64>,

    /// Nucleus sampling parameter.
    ///
    /// Nucleus sampling considers the smallest set of tokens whose
    /// probability sum is at least topP. This value specifies default
    /// to be used by the backend while making the call to the model.
    pub top_p: Option<f64>,

    /// Top-k sampling parameter.
    ///
    /// Top-k sampling considers the set of topK most probable
    /// tokens. This value specifies default to be used by the backend
    /// while making the call to the model. If empty, indicates the
    /// model doesn't use top-k sampling, and topK isn't allowed as a
    /// generation parameter.
    pub top_k: Option<i64>,
}
