use serde::Deserialize;

/// OpenAI model description.
///
/// Describes an OpenAI model offering that can be used with the API.
///
/// See: [API Reference](https://platform.openai.com/docs/api-reference/models/object)
#[derive(Deserialize, Debug)]
pub struct Model {

    /// The Unix timestamp (in seconds) when the model was created.
    pub created: i64,

    /// The model identifier, which can be referenced in the API
    /// endpoints.
    pub id: String,

    /// The object type, which is always "model".
    pub object: String,

    /// The organization that owns the model.
    pub owned_by: String,
}
