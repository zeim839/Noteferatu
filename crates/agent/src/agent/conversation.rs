use serde::{Serialize, Deserialize};
use crate::core::Message;
use sqlx::FromRow;

/// Indexes a list of message requests and completion responses.
///
/// A conversation's `id` can be used to fetch an associated message
/// history.
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {

    /// Unique identifier for the conversation.
    pub id: i64,

    /// Name of the conversation. Can be non-unique.
    pub name: String,

    /// Timestamp of the conversation's creation.
    pub created_at: i64,
}

#[derive(FromRow)]
pub(crate) struct MessageEntry {
    pub id: i64,
    pub conv_id: i64,
    pub object: String,
}

impl Into<Message> for MessageEntry {
    fn into(self) -> Message {
        serde_json::from_str(&self.object).unwrap()
    }
}
