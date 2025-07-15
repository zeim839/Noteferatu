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
}
