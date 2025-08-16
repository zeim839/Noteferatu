use super::file::DriveFile;
use crate::core::File;

use serde::{Serialize, Deserialize};
use chrono::DateTime;

/// A change to a file or shared drive.
///
/// Reference: [Change Resource](https://developers.google.com/workspace/drive/api/reference/rest/v3/changes)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveChange {

    /// Identifies what kind of resource this is. Value: the fixed
    /// string "drive#change"
    pub kind: String,

    /// Whether the file or shared drive has been removed from this
    /// list of changes, for example by deletion or loss of access.
    pub removed: bool,

    /// The updated state of the file. Present if the type is file and
    /// the file has not been removed from this list of changes.
    pub file: Option<DriveFile>,

    /// The ID of the file which has changed.
    pub file_id: String,

    /// The time of this change (RFC 3339 date-time).
    pub time: String,

    /// The ID of the shared drive associated with this change.
    pub drive_id: Option<String>,
}

impl Into<File> for DriveChange {
    fn into(self) -> File {
        let modified_at = DateTime::parse_from_rfc3339(&self.time.clone())
            .map(|dt| dt.timestamp())
            .unwrap_or(0);

        let created_at = self.file.clone().map(|file| match file.created_time {
            Some(ts) => DateTime::parse_from_rfc3339(&ts)
                .map(|dt| dt.timestamp())
                .unwrap_or(0),
            None => 0,
        }).unwrap_or_default();

        let parent_id = self.file.clone().map(|file| match file.parents {
            Some(parents) => parents.get(0).map(|t| t.clone()),
            None => None,
        }).unwrap_or(None);

        let is_folder = self.file.clone()
            .map(|file| file.mime_type.unwrap_or(String::new()))
            .map(|m| m == "application/vnd.google-apps.folder")
            .unwrap_or(false);

        File {
            id: self.file_id,
            modified_at,
            created_at,
            parent_id,
            is_folder,
        }
    }
}
