use tauri::{AppHandle, command, Runtime};

use super::models::*;
use super::Result;
use super::HelsyncExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.helsync().ping(payload)
}
