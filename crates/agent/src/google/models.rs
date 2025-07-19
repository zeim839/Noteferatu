use serde::{Serialize, Deserialize};

/// A Google AI model definition.
///
/// API Reference: [Model](https://ai.google.dev/api/models#Model)
#[derive(Serialize, Deserialize)]
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
    #[serde(rename = "displayName")]
    pub display_name: String,

    /// Maximum number of input tokens allowed for this model.
    #[serde(rename = "inputTokenLimit")]
    pub input_token_limit: i64,

    /// Maximum number of output tokens available for this model.
    #[serde(rename = "outputTokenLimit")]
    pub output_token_limit: i64,

    /// The model's supported generation methods.
    #[serde(rename = "supportedGenerationMethods")]
    pub supported_generation_methods: Vec<String>,

    /// Whether the model supports thinking.
    pub thinking: Option<bool>,

    /// Controls the randomness of the output.
    pub temperature: Option<f64>,

    /// The maximum temperature this model can use.
    #[serde(rename = "maxTemperature")]
    pub max_temperature: Option<f64>,

    /// Nucleus sampling.
    ///
    /// Nucleus sampling considers the smallest set of tokens whose
    /// probability sum is at least `top_p`. This value specifies default
    /// to be used by the backend while making the call to the model.
    #[serde(rename = "topP")]
    pub top_p: Option<f64>,

    /// For Top-k sampling.
    ///
    /// Top-k sampling considers the set of topK most probable
    /// tokens. This value specifies default to be used by the backend
    /// while making the call to the model. If empty, indicates the model
    /// doesn't use top-k sampling, and topK isn't allowed as a generation
    /// parameter.
    #[serde(rename = "topK")]
    pub top_k: Option<f64>,
}

impl crate::ModelDefinition for Model {
    fn id(&self) -> String {
        self.name.clone()
    }

    fn display_name(&self) -> String {
        self.display_name.clone()
    }

    fn context_length(&self) -> u64 {
        self.input_token_limit as u64
    }

    fn supports_tool_calls(&self) -> bool {
        match self.name.as_str() {
            "gemini-2.5-pro" => true,
            "gemini-2.5-flash" => true,
            "gemini-2.5-flash-lite-preview-06-17" => true,
            "gemini-2.0-flash" => true,
            "gemini-2.0-flash-preview-image-generation" => true,
            _ => false,
        }
    }

    fn supports_web_search(&self) -> bool {
        true
    }
}
