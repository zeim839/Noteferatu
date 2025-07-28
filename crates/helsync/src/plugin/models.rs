use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetFileRequest {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CopyFileRequest {
    pub source_id: String,
    pub parent_id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MoveFileRequest {
    pub source_id: String,
    pub parent_id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFileRequest {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest {
    pub parent_id: Option<String>,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListFilesRequest {
    pub parent_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WriteToFileRequest {
    pub parent_id: Option<String>,
    pub name: String,
    pub contents: Vec<u8>,
}

/// Describes a change made to files in the filesystem.
///
/// Sent as a [global tauri event](https://v2.tauri.app/develop/calling-frontend/#global-events)
/// whenever a filesystem change is detected.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all="camelCase", rename_all_fields="camelCase", tag="event", content="data")]
pub enum FsChangeEvent {
    Copy(CopyFileRequest),
    Move(MoveFileRequest),
    Remove(RemoveFileRequest),
    CreateFolder(CreateFolderRequest),
}
