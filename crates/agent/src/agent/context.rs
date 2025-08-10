use super::conversation::{MessageEntry, Conversation};
use super::agent::Agent;
use crate::core::*;

use std::sync::Arc;
use tokio::sync::{RwLock, oneshot, Mutex};
use database::Database;
use serde_json::json;
use sqlx::QueryBuilder;
use std::mem;

/// Exposes an [Agent](super::Agent) handling a conversation thread.
///
/// Automatically populates chat completion requests with message
/// history and writes new messages to the conversation [database].
pub struct Context {
    db: Arc<Database>,
    agent: Arc<RwLock<Agent>>,
    conv: Conversation,
    cancel: Mutex<Option<oneshot::Sender<()>>>,
}

impl Context {

    /// Create a new [Context] instance.
    pub(crate) fn new(db: Arc<Database>, agent: Arc<RwLock<Agent>>, conv: Conversation) -> Self {
        Self {cancel: Mutex::new(None), db, agent, conv}
    }

    /// Stop all chat completion requests.
    pub async fn stop_messages(&self) {
        if let Some(tx) = self.cancel.lock().await.take() {
            let _ = tx.send(());
        }
    }

    /// Send a non-streaming message to the conversation thread.
    pub async fn send_message(&self, req: Request) -> Result<Response> {
        let mut cancel_guard = self.cancel.lock().await;

        // Cancel any pending messages.
        if let Some(tx) = cancel_guard.take() {
            let _ = tx.send(());
        }

        // Register cancellation channel.
        let (tx, rx) = oneshot::channel::<()>();
        *cancel_guard = Some(tx);
        drop(cancel_guard);

        let mut agent_req_messages = self.list_messages().await?;
        agent_req_messages.extend(req.messages.clone());

        let agent = self.agent.read().await;
        let agent_req = Request { messages: agent_req_messages, ..req.clone() };

        // Wait for either the kill signal or the chat completion to
        // finish. If the kill signal arrives first, the chat
        // completion is dropped.
        let result = tokio::select! {
            res = agent.completion(agent_req) => res.map(Some),
            _ = rx => Ok(None),
        };

        match result {
            Ok(Some(res)) => {
                let all_new_messages: Vec<Message> = req.messages
                    .into_iter()
                    .chain(res.messages.clone().into_iter())
                    .collect();

                if !all_new_messages.is_empty() {
                    let mut conn = self.db.acquire().await?;
                    let mut builder = QueryBuilder::new("INSERT INTO Message (conv_id, object) ");
                    builder.push_values(all_new_messages, |mut b, message| {
                        b.push_bind(self.conv.id)
                         .push_bind(json!(message).to_string());
                    });
                    builder.build().execute(&mut *conn).await?;
                }
                Ok(res)
            }
            Ok(None) => Ok(Response::default()), // Canceled
            Err(e) => Err(e), // Agent error
        }
    }

    /// Send a streaming message to the conversation thread.
    pub async fn send_stream_message<F>(&self, req: Request, mut cb: F) -> Result<Response>
    where F: FnMut(Response) {
        let mut cancel_guard = self.cancel.lock().await;

        // Cancel any pending messages.
        if let Some(tx) = cancel_guard.take() {
            let _ = tx.send(());
        }

        // Register cancellation channel.
        let (tx, rx) = oneshot::channel::<()>();
        *cancel_guard = Some(tx);
        drop(cancel_guard);

        let mut agent_req_messages = self.list_messages().await?;
        agent_req_messages.extend(req.messages.clone());

        let agent_req = Request { messages: agent_req_messages, ..req.clone() };
        let mut completed_messages: Vec<Message> = Vec::new();
        let mut accumulated_message: Option<Message> = None;
        let agent = self.agent.read().await;

        // Wait for either the kill signal or the chat completion to
        // finish. If the kill signal arrives first, the chat
        // completion is dropped.
        tokio::select! {
            _ = agent.stream_completion(agent_req, |event| {
                cb(event.clone());
                for msg in event.messages {
                    let new_msg_starts =
                        if let Some(acc_msg) = &accumulated_message {
                            if mem::discriminant(&acc_msg.content) == mem::discriminant(&msg.content) {
                                !matches!(&acc_msg.content, MessageContent::Text(_))
                            } else {
                                true
                            }
                        } else {
                            true
                        };

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
            }) => {}
            _ = rx => {}
        }

        if let Some(last_msg) = accumulated_message.take() {
            completed_messages.push(last_msg);
        }

        let all_new_messages: Vec<Message> = req.messages
            .into_iter()
            .chain(completed_messages.clone().into_iter())
            .collect();

        if !all_new_messages.is_empty() {
            let mut conn = self.db.acquire().await?;
            let mut builder = QueryBuilder::new("INSERT INTO Message (conv_id, object) ");
            builder.push_values(all_new_messages, |mut b, message| {
                b.push_bind(self.conv.id)
                    .push_bind(json!(message).to_string());
            });
            builder.build().execute(&mut *conn).await?;
        }

        let mut res = Response::default();
        res.messages = completed_messages;

        Ok(res)
    }

    /// Retrieve all messages in the
    /// [Conversation](super::Conversation).
    pub async fn list_messages(&self)  -> Result<Vec<Message>> {
        let mut conn = self.db.acquire().await?;
        let msgs: Vec<MessageEntry> = sqlx::query_as("SELECT * FROM Message WHERE conv_id=?")
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
        let ctx = manager.get_conversation(0).await.unwrap();
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
        let ctx = manager.get_conversation(conv.id).await.unwrap();
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
