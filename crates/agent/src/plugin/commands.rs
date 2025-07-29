use tauri::{AppHandle, command, Runtime};

use super::models::*;
use super::Result;
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
    app: AppHandle<R>,
    payload: ListModelsRequest,
) -> Result<Vec<crate::Model>> {
    app.agent().list_models(payload).await
}
