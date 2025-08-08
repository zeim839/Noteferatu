use serde::{Deserialize, Serialize};
use crate::core::Response;

/// A Tauri IPC event for streaming chat completions.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "event", content = "data")]
pub enum StreamEvent {
    Started,
    Content(Response),
    Finished,
}
