use serde::de::DeserializeOwned;
use tauri::plugin::PluginApi;
use tauri::{AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use tokio::sync::Mutex;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> Result<Agent<R>> {
    Ok(Agent {
        app: app.clone(),
        client: Mutex::new(agent::Agent::new()),
    })
}

/// Access to the agent APIs.
pub struct Agent<R: Runtime> {
    app: AppHandle<R>,
    client: Mutex<agent::Agent>,
}

impl<R: Runtime> Agent<R> {
    pub async fn try_connect(&self, payload: TryConnectRequest) -> Result<()> {
        let mut client = self.client.lock().await;
        match payload.provider.as_str() {
            "anthropic" => client.register_anthropic(&payload.api_key).await?,
            "google" => client.register_google(&payload.api_key).await?,
            "ollama" => client.register_ollama(&payload.api_key).await?,
            "openai" => client.register_openai(&payload.api_key).await?,
            "openrouter" => client.register_openrouter(&payload.api_key).await?,
            _ => panic!("unknown provider"),
        }
        Ok(())
    }
}
