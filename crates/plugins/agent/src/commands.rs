use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::AgentExt;

#[command]
pub(crate) async fn try_connect<R: Runtime>(
    app: AppHandle<R>,
    payload: TryConnectRequest,
) -> Result<()> {
    app.agent().try_connect(payload).await
}
