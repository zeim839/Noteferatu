use serde::{Deserialize, Serialize};
use crate::core::File;
use chrono::DateTime;

/// The metadata for a file.
///
/// Reference: [Files Resource](https://developers.google.com/workspace/drive/api/reference/rest/v3/files)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveFile {

    /// Output only. The final component of `fullFileExtension`. This is
    /// only available for files with binary content in Google Drive.
    pub file_extension: Option<String>,

    /// Output only. The MD5 checksum for the content of the
    /// file. This is only applicable to files with binary content in
    /// Google Drive.
    pub md5_checksum: Option<String>,

    /// The MIME type of the file.
    ///
    /// Google Drive attempts to automatically detect an appropriate
    /// value from uploaded content, if no value is provided. The
    /// value cannot be changed unless a new revision is uploaded.
    ///
    /// If a file is created with a Google Doc MIME type, the uploaded
    /// content is imported, if possible. The supported import formats
    /// are published in the About resource.
    pub mime_type: Option<String>,

    /// The ID of the parent folder containing the file.
    ///
    /// A file can only have one parent folder; specifying multiple
    /// parents isn't supported.
    ///
    /// If not specified as part of a create request, the file is
    /// placed directly in the user's My Drive folder. If not
    /// specified as part of a copy request, the file inherits any
    /// discoverable parent of the source file. files.update requests
    /// must use the addParents and removeParents parameters to modify
    /// the parents list.
    pub parents: Option<Vec<String>>,

    /// Output only. Size in bytes of blobs and first party editor
    /// files. Won't be populated for files that have no size, like
    /// shortcuts and folders.
    pub size: Option<String>,

    /// The ID of the file.
    pub id: String,

    /// The name of the file. This is not necessarily unique within a
    /// folder. Note that for immutable items such as the top level
    /// folders of shared drives, My Drive root folder, and
    /// Application Data folder the name is constant.
    pub name: String,

    /// Whether the file has been trashed, either explicitly or from a
    /// trashed parent folder. Only the owner may trash a file, and
    /// other users cannot see files in the owner's trash.
    pub trashed: Option<bool>,

    /// Output only. Whether the file has been explicitly trashed, as
    /// opposed to recursively trashed from a parent folder.
    pub explicitly_trashed: Option<bool>,

    /// The time at which the file was created (RFC 3339 date-time).
    pub created_time: Option<String>,

    /// The last time the file was modified by anyone (RFC 3339
    /// date-time).
    pub modified_time: Option<String>,

    /// Output only. A monotonically increasing version number for the
    /// file. This reflects every change made to the file on the
    /// server, even those not visible to the user.
    pub version: Option<String>,

    /// Output only. The time that the item was trashed (RFC 3339
    /// date-time). Only populated for items in shared drives.
    pub trashed_time: Option<String>,

    /// Output only. The SHA1 checksum associated with this file, if
    /// available. This field is only populated for files with content
    /// stored in Google Drive; it is not populated for Docs Editors
    /// or shortcut files.
    pub sha1_checksum: Option<String>,

    /// Output only. The SHA256 checksum associated with this file, if
    /// available. This field is only populated for files with content
    /// stored in Google Drive; it is not populated for Docs Editors
    /// or shortcut files.
    pub sha256_checksum: Option<String>,
}

impl Into<File> for DriveFile {
    fn into(self) -> File {
        let modified_at = match self.modified_time {
            Some(ts) => DateTime::parse_from_rfc3339(&ts)
                .map(|dt| dt.timestamp())
                .unwrap_or(0),
            None => 0,
        };

        let created_at = match self.created_time {
            Some(ts) => DateTime::parse_from_rfc3339(&ts)
                .map(|dt| dt.timestamp())
                .unwrap_or(0),
            None => 0,
        };

        let parent_id = self.parents
            .map(|p| p.get(0).map(|t| t.clone()))
            .unwrap_or(None);

        let is_folder = self.mime_type
            .map(|m| m == "application/vnd.google-apps.folder")
            .unwrap_or(false);

        File {
            id: self.id,
            name: self.name,
            modified_at,
            created_at,
            parent_id,
            is_folder,
            is_deleted: self.trashed.is_some_and(|b| b),
        }
    }
}
