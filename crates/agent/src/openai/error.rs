use serde::{Serialize, Deserialize};

/// An OpenAI error.
///
/// API Reference: [Error Codes](https://platform.openai.com/docs/guides/error-codes)
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorAPI {
    pub message: String,
    #[serde(rename = "type", default)]
    pub kind: String,
    #[serde(default, deserialize_with = "from_str")]
    pub code: String,
}

impl std::fmt::Display for ErrorAPI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) {}", self.code, self.message)
    }
}

fn from_str<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    let s: serde_json::Value = Deserialize::deserialize(deserializer)?;
    Ok(s.to_string().trim_matches('"').to_string())
}
