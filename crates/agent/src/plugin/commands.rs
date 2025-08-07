use tauri::{AppHandle, command, Runtime};

use crate::core::{Result, Model};
use super::models::*;
use super::AgentExt;

#[command]
pub(crate) async fn try_connect<R: Runtime>(
    app: AppHandle<R>,
    payload: TryConnectRequest,
) -> Result<()> {
    app.agent().try_connect(payload).await
}

#[command]
pub(crate) async fn list_models<R: Runtime>(
    app: AppHandle<R>
) -> Result<Vec<Model>> {
    app.agent().list_models().await
}
