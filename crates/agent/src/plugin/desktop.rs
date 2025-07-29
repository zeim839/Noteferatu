use serde::de::DeserializeOwned;
use tauri::plugin::PluginApi;
use tauri::{AppHandle, Runtime};

use super::models::*;
use super::Result;
use tokio::sync::Mutex;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> Result<Agent<R>> {
    Ok(Agent {
        _app: app.clone(),
        client: Mutex::new(crate::Agent::new()),
    })
}

/// Access to the agent APIs.
pub struct Agent<R: Runtime> {
    _app: AppHandle<R>,
    client: Mutex<crate::Agent>,
}

impl<R: Runtime> Agent<R> {
    pub async fn try_connect(&self, payload: TryConnectRequest) -> Result<()> {
        let mut client = self.client.lock().await;
        match payload.provider.to_lowercase().as_str() {
            "anthropic" => client.register_anthropic(&payload.api_key).await?,
            "google" => client.register_google(&payload.api_key).await?,
            "ollama" => client.register_ollama(&payload.api_key).await
                .map_err(|_| crate::Error{
                    // Use custom error message.
                    kind: "OLLAMA_ERR".to_string(),
                    message: "invalid connection URL".to_string(),
                })?,
            "openai" => client.register_openai(&payload.api_key).await?,
            "openrouter" => client.register_openrouter(&payload.api_key).await?,
            _ => panic!("unknown provider"),
        }
        Ok(())
    }

    pub async fn list_models(&self, payload: ListModelsRequest) -> Result<Vec<crate::Model>> {
        let client = self.client.lock().await;
        let models = client.list_models(payload.provider.as_deref()).await?;
        Ok(models)
    }
}
