use super::conversation::{MessageEntry, Conversation};
use super::agent::Agent;
use crate::core::*;

use std::sync::Arc;
use tokio::sync::RwLock;
use database::Database;
use sqlx::Acquire;
use serde_json::json;
use std::mem;

/// Exposes an [Agent](super::Agent) handling a conversation thread.
///
/// Automatically populates chat completion requests with message
/// history and writes new messages to the conversation [database].
pub struct Context {
    db: Arc<Database>,
    agent: Arc<RwLock<Agent>>,
    history: Vec<Message>,
    conv: Conversation,
}

impl Context {

    /// Create a new [Context] instance.
    pub(crate) fn new(db: Arc<Database>, agent: Arc<RwLock<Agent>>, conv: Conversation) -> Self {
        Self { db, agent, history: Vec::new(), conv }
    }

    /// Send a non-streaming message to the conversation thread.
    pub async fn send_message(&mut self, req: Request) -> Result<Response> {
        let mut conn = self.db.acquire().await?;
        let mut tx = conn.begin().await?;
        for message in req.messages.clone() {
            sqlx::query("INSERT INTO Message(conv_id, object) VALUES (?,?)")
                .bind(self.conv.id)
                .bind(json!(message).to_string())
                .execute(&mut *tx)
                .await?;

            self.history.push(message);
        }

        let req = Request { messages: self.history.clone(), ..req };
        let res = self.agent.read().await.completion(req).await?;
        for message in res.messages.clone() {
            sqlx::query("INSERT INTO Message(conv_id, object) VALUES (?, ?)")
                .bind(self.conv.id)
                .bind(json!(message).to_string())
                .execute(&mut *tx)
                .await?;

            self.history.push(message);
        }

        tx.commit().await?;
        Ok(res)
    }

    /// Send a streaming message to the conversation thread.
    pub async fn send_stream_message<F>(&mut self, req: Request, mut cb: F) -> Result<Response>
    where F: FnMut(Response) {
        let mut conn = self.db.acquire().await?;
        let mut tx = conn.begin().await?;
        for message in req.messages.clone() {
            sqlx::query("INSERT INTO Message(conv_id, object) VALUES(?,?)")
                .bind(self.conv.id)
                .bind(json!(message).to_string())
                .execute(&mut *tx)
                .await?;

            self.history.push(message);
        }

        let req = Request { messages: self.history.clone(), ..req };
        let mut completed_messages: Vec<Message> = Vec::new();
        let mut accumulated_message: Option<Message> = None;

        self.agent.read().await.stream_completion(req, |event| {
            cb(event.clone());
            for msg in event.messages {
                let new_msg_starts = if let Some(acc_msg) = &accumulated_message {
                    if mem::discriminant(&acc_msg.content) == mem::discriminant(&msg.content) {
                        !matches!(&acc_msg.content, MessageContent::Text(_))
                    } else { true }
                } else { true };
                if new_msg_starts {
                    if let Some(complete_msg) = accumulated_message.take() {
                        completed_messages.push(complete_msg);
                    }
                    accumulated_message = Some(msg);
                } else {
                    if let Some(acc_msg) = &mut accumulated_message {
                        if let (MessageContent::Text(acc_text), MessageContent::Text(new_text)) = (&mut acc_msg.content, msg.content) {
                            acc_text.push_str(&new_text);
                        }
                    }
                }
            }
        }).await?;

        if let Some(last_msg) = accumulated_message.take() {
            completed_messages.push(last_msg);
        }

        for message in completed_messages.clone() {
            sqlx::query("INSERT INTO Message(conv_id, object) VALUES (?, ?)")
                .bind(self.conv.id)
                .bind(json!(message).to_string())
                .execute(&mut *tx)
                .await?;

            self.history.push(message);
        }

        tx.commit().await?;

        // TODO: INCLUDE ACCUMULATED USAGE INFORMATION.
        let mut res = Response::default();
        res.messages = completed_messages;
        Ok(res)
    }

    /// Retrieve all messages in the
    /// [Conversation](super::Conversation).
    pub async fn list_messages(&self)  -> Result<Vec<Message>> {
        let mut conn = self.db.acquire().await?;
        let msgs: Vec<MessageEntry> = sqlx::query_as("SELECT * FROM
Message WHERE conv_id=?")
            .bind(self.conv.id)
            .fetch_all(&mut *conn)
            .await?;

        Ok(msgs.into_iter().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::manager::tests::get_manager;
    use crate::core::MessageContent;

    #[tokio::test]
    async fn test_send_message() {
        let manager = get_manager().await;
        let mut ctx = manager.get_conversation(0).await.unwrap();
        let req = Request {
            model: "openrouter:deepseek/deepseek-r1-0528:free".to_string(),
            messages: vec![Message {
                role: Role::User,
                content: MessageContent::Text("Hello".to_string()),
            }],
            max_tokens: Some(300),
            system: None,
            tools: vec![],
        };
        let res = ctx.send_message(req).await.unwrap();
        let mut has_text_response = false;
        for message in res.messages {
            if let MessageContent::Text(_) = message.content {
                has_text_response = true;
                break;
            }
        }
        assert!(has_text_response);
    }

    #[tokio::test]
    async fn test_send_stream_message() {
        let manager = get_manager().await;
        let conv = manager.create_conversation("new-conversation").await.unwrap();
        let mut ctx = manager.get_conversation(conv.id).await.unwrap();
        let req = Request {
            model: "openrouter:deepseek/deepseek-r1-0528:free".to_string(),
            messages: vec![Message {
                role: Role::User,
                content: MessageContent::Text("Hello".to_string()),
            }],
            max_tokens: Some(300),
            system: None,
            tools: vec![],
        };
        ctx.send_stream_message(req, |_| {}).await.unwrap();
        let messages = ctx.list_messages().await.unwrap();
        assert!(messages.len() > 0);
    }
}
