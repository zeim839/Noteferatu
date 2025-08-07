use crate::core::{Error, Result, Model};
use crate::agent::{Conversation, Manager};
use super::models::*;

use database::Database;
use serde::de::DeserializeOwned;
use tauri::plugin::PluginApi;
use tauri::{AppHandle, Runtime};
use tokio::sync::Mutex;
use std::sync::Arc;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
    db: Arc<Database>,
) -> Result<Agent<R>> {
    Ok(Agent {
        _app: app.clone(),
        manager: Arc::new(Mutex::new(Manager::new(db))),
    })
}

/// Access to the agent APIs.
pub struct Agent<R: Runtime> {
    _app: AppHandle<R>,
    manager: Arc<Mutex<Manager>>,
}

impl<R: Runtime> Agent<R> {

    pub async fn try_connect(&self, payload: TryConnectRequest) -> Result<()> {
        let mut manager = self.manager.lock().await;
        match payload.provider.to_lowercase().as_str() {
            "anthropic" => manager.connect_anthropic(&payload.api_key).await?,
            "google" => manager.connect_google(&payload.api_key).await?,
            "ollama" => manager.connect_ollama(&payload.api_key).await
                .map_err(|_| Error::Ollama("invalid connection URL".to_string()))?,
            "openai" => manager.connect_openai(&payload.api_key).await?,
            "openrouter" => manager.connect_openrouter(&payload.api_key).await?,
            _ => panic!("unknown provider"),
        }
        Ok(())
    }

    pub async fn list_models(&self) -> Result<Vec<Model>> {
        let manager = self.manager.lock().await;
        Ok(manager.list_models().await?)
    }

    pub async fn list_conversations(&self) -> Result<Vec<Conversation>> {
        let manager = self.manager.lock().await;
        Ok(manager.list_conversations().await?)
    }

    pub async fn create_conversation(&self, payload: CreateConversationRequest) -> Result<Conversation> {
        let manager = self.manager.lock().await;
        Ok(manager.create_conversation(&payload.name).await?)
    }

    pub async fn rename_conversation(&self, payload: RenameConversationRequest) -> Result<()> {
        let manager = self.manager.lock().await;
        Ok(manager.rename_conversation(payload.id, &payload.new_name).await?)
    }

    pub async fn remove_conversation(&self, payload: RemoveConversationRequest) -> Result<()> {
        let manager = self.manager.lock().await;
        Ok(manager.remove_conversation(payload.id).await?)
    }
}
