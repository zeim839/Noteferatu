use tauri::{AppHandle, command, Runtime, Emitter};
use tauri::ipc::Channel;

use crate::core::{Result, Model, Request, Response};
use crate::agent::Conversation;
use super::models::*;
use super::AgentExt;

#[command]
pub(crate) async fn try_connect<R: Runtime>(
    app: AppHandle<R>,
    provider: String,
    api_key: String,
) -> Result<()> {
    app.agent().try_connect(provider, api_key).await
}

#[command]
pub(crate) async fn list_models<R: Runtime>(
    app: AppHandle<R>,
    provider: Option<String>,
) -> Result<Vec<Model>> {
    app.agent().list_models(provider).await
}

#[command]
pub(crate) async fn list_conversations<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<Conversation>> {
    app.agent().list_conversations().await
}

#[command]
pub(crate) async fn create_conversation<R: Runtime>(
    app: AppHandle<R>,
    name: String,
) -> Result<Conversation> {
    app.emit("agent-conversations-change", "")?;
    app.agent().create_conversation(name).await
}

#[command]
pub(crate) async fn rename_conversation<R: Runtime>(
    app: AppHandle<R>,
    id: i64,
    new_name: String,
) -> Result<()> {
    app.emit("agent-conversations-change", "")?;
    app.agent().rename_conversation(id, new_name).await
}

#[command]
pub(crate) async fn remove_conversation<R: Runtime>(
    app: AppHandle<R>,
    id: i64,
) -> Result<()> {
    app.emit("agent-conversations-change", "")?;
    app.agent().remove_conversation(id).await
}

#[command]
pub(crate) async fn send_message<R: Runtime>(
    app: AppHandle<R>,
    conversation_id: i64,
    request: Request,
) -> Result<Response> {
    app.agent().send_message(conversation_id, request).await
}

#[command]
pub(crate) async fn send_stream_message<R: Runtime>(
    app: AppHandle<R>,
    conversation_id: i64,
    request: Request,
    channel: Channel<StreamEvent>,
) -> Result<Response> {
    app.agent().send_stream_message(conversation_id, request, channel).await
}
