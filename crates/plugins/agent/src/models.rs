use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TryConnectRequest {
    pub provider: String,
    pub api_key: String,
}
