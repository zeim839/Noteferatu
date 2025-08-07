use tauri::{AppHandle, command, Runtime, Emitter};

use crate::core::{Result, Model};
use crate::agent::Conversation;
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

#[command]
pub(crate) async fn list_conversations<R: Runtime>(
    app: AppHandle<R>
) -> Result<Vec<Conversation>> {
    app.agent().list_conversations().await
}

#[command]
pub(crate) async fn create_conversation<R: Runtime>(
    app: AppHandle<R>,
    payload: CreateConversationRequest,
) -> Result<Conversation> {
    app.emit("agent-conversations-change", "")?;
    app.agent().create_conversation(payload).await
}

#[command]
pub(crate) async fn rename_conversation<R: Runtime>(
    app: AppHandle<R>,
    payload: RenameConversationRequest,
) -> Result<()> {
    app.emit("agent-conversations-change", "")?;
    app.agent().rename_conversation(payload).await
}

#[command]
pub(crate) async fn remove_conversation<R: Runtime>(
    app: AppHandle<R>,
    payload: RemoveConversationRequest,
) -> Result<()> {
    app.emit("agent-conversations-change", "")?;
    app.agent().remove_conversation(payload).await
}
