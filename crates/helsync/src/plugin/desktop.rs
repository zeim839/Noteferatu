use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use super::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> super::Result<Helsync<R>> {
    Ok(Helsync(app.clone()))
}

/// Access to the helsync APIs.
pub struct Helsync<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Helsync<R> {
    pub fn ping(&self, payload: PingRequest) -> super::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }
}
