use crate::fs::Delta;
use super::file::DriveFile;
use serde::{Deserialize, Serialize};

/// A change to a file or shared drive.
///
/// Reference: [Change Resource](https://developers.google.com/workspace/drive/api/reference/rest/v3/changes)
#[derive(Serialize, Deserialize)]
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
    #[serde(rename = "fileId")]
    pub file_id: String,

    /// The time of this change (RFC 3339 date-time).
    pub time: String,

    /// The ID of the shared drive associated with this change.
    #[serde(rename = "driveId")]
    pub drive_id: Option<String>,

}

impl Delta for DriveChange {
    fn id(&self) -> String {
        self.file_id.clone()
    }

    fn is_removed(&self) -> bool {
        self.removed
    }

    fn is_modified(&self) -> bool {
        self.file.is_some()
    }
}
