use crate::core::{Error, Result, Model, Request, Response, Message};
use crate::agent::{Conversation, Manager};
use super::models::*;

use tauri::plugin::PluginApi;
use tauri::{AppHandle, Runtime};
use tauri::ipc::Channel;

use database::Database;
use serde::de::DeserializeOwned;
use std::sync::Arc;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
    db: Arc<Database>,
) -> Result<Agent<R>> {
    Ok(Agent {
        _app: app.clone(),
        manager: Arc::new(Manager::new(db)),
    })
}

/// Access to the agent APIs.
pub struct Agent<R: Runtime> {
    _app: AppHandle<R>,
    manager: Arc<Manager>,
}

impl<R: Runtime> Agent<R> {

    /// Checks if `api_key` works for provider and then tries to
    /// connect.
    ///
    /// If the connection attempt is successful, then the provider's
    /// LLM models can be used in subsequent commands
    /// (e.g. list_models, etc.).
    pub async fn try_connect(&self, provider: String, api_key: String) -> Result<()> {
        match provider.to_lowercase().as_str() {
            "anthropic" => self.manager.connect_anthropic(&api_key).await?,
            "google" => self.manager.connect_google(&api_key).await?,
            "ollama" => self.manager.connect_ollama(&api_key).await
                .map_err(|_| Error::Ollama("invalid connection URL".to_string()))?,
            "openai" => self.manager.connect_openai(&api_key).await?,
            "openrouter" => self.manager.connect_openrouter(&api_key).await?,
            _ => panic!("unknown provider"),
        }
        Ok(())
    }

    /// List all available LLM models.
    ///
    /// Lists the LLM models available via the connected providers
    /// (see [try_connect](Self::try_connect). If `provider` is
    /// specified, then only the given provider's models are returned.
    pub async fn list_models(&self, provider: Option<String>) -> Result<Vec<Model>> {
        let mut models = self.manager.list_models().await?;
        if let Some(provider) = provider {
            models = models.into_iter()
                .filter(|item| item.provider == provider)
                .collect();

            // Remove non-text models from certain providers.
            if provider.to_lowercase() == "anthropic" {
                models = models.into_iter()
                    .filter(|item| item.id.starts_with("claude"))
                    .collect();
            }

            if provider.to_lowercase() == "google" {
                models = models.into_iter()
                    .filter(|item| item.id.starts_with("gemini") ||
                            item.id.starts_with("gemma"))
                    .collect();
            }

        }
        Ok(models)
    }

    /// List the conversation history.
    pub async fn list_conversations(&self) -> Result<Vec<Conversation>> {
        Ok(self.manager.list_conversations().await?)
    }

    /// Create a new conversation.
    pub async fn create_conversation(&self, name: String) -> Result<Conversation> {
        Ok(self.manager.create_conversation(&name).await?)
    }

    /// Rename a conversation.
    pub async fn rename_conversation(&self, id: i64, new_name: String) -> Result<()> {
        Ok(self.manager.rename_conversation(id, &new_name).await?)
    }

    /// Delete a conversation and all its messages.
    pub async fn remove_conversation(&self, id: i64) -> Result<()> {
        Ok(self.manager.remove_conversation(id).await?)
    }

    /// Send a non-streaming message to a conversation thread.
    pub async fn send_message(&self, conversation_id: i64, request: Request) -> Result<Response> {
        let res = self.manager.get_conversation(conversation_id).await?
            .send_message(request).await?;

        Ok(res)
    }

    /// Send a streaming message to a conversation thread.
    ///
    /// Sends a series of [StreamEvent](super::models::StreamEvent] as
    /// messages arrive from the provider. Once finished, it returns
    /// the final chat response.
    pub async fn send_stream_message(&self, conversation_id: i64, request: Request, chan: Channel<StreamEvent>) -> Result<Response> {
        chan.send(StreamEvent::Started)?;
        let res = self.manager.get_conversation(conversation_id).await?
            .send_stream_message(request, |event| {
                chan.send(StreamEvent::Content(event)).unwrap();
            })
            .await?;

        chan.send(StreamEvent::Finished)?;
        Ok(res)
    }

    /// List the messages in the specified conversation.
    pub async fn list_messages(&self, conversation_id: i64) -> Result<Vec<Message>> {
        let messages = self.manager.get_conversation(conversation_id).await?
            .list_messages().await?;

        Ok(messages)
    }

    /// Stop all chat completion requests.
    pub async fn stop_messages(&self, conversation_id: i64) -> Result<()> {
        self.manager.get_conversation(conversation_id).await?
            .stop_messages().await;

        Ok(())
    }
}
