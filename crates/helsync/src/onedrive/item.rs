use crate::filesystem::{File, Delta};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset};
use serde_json::Value;

/// The driveItem resource represents a file, folder, or other item
/// stored in a drive. All file system objects in OneDrive and
/// SharePoint are returned as driveItem resources.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveItem {

    /// Date and time of item creation. Read-only.
    pub created_date_time: Option<String>,

    /// Information about the deleted state of the item. Read-only.
    pub deleted: Option<DeletedMetadata>,

    /// File metadata, if the item is a file. Read-only.
    pub file: Option<FileMetadata>,

    /// Folder metadata, if the item is a folder. Read-only.
    pub folder: Option<FolderMetadata>,

    /// The unique identifier of the item within the Drive. Read-only.
    pub id: String,

    /// Date and time the item was last modified. Read-only.
    pub last_modified_date_time: Option<String>,

    /// The name of the item (filename and extension). Read-write.
    pub name: Option<String>,

    /// Parent information, if the item has a parent. Read-write.
    pub parent_reference: Option<ItemReference>,

    /// If this property is non-null, it indicates that the driveItem
    /// is the top-most driveItem in the drive.
    pub root: Option<Value>,

    /// Size of the item in bytes. Read-only.
    pub size: Option<i64>,
}

/// The DeletedMetadata resource indicates that the item has been
/// deleted.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedMetadata {

    /// Represents the state of the deleted item.
    pub state: Option<String>,
}

/// The FileMetadata resource groups file-related data items into a
/// single structure.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadata {

    /// The MIME type for the file. This is determined by logic on the
    /// server and might not be the value provided when the file was
    /// uploaded. Read-only.
    pub mime_type: Option<String>,
}

/// The Folder resource groups folder-related data on an item into a
/// single structure. [DriveItems](DriveItem) with a non-null folder
/// facet are containers for other [DriveItems](DriveItem).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderMetadata {

    /// Number of children contained immediately within this
    /// container.
    pub child_count: Option<i32>,
}

/// The ItemReference resource provides information necessary to
/// address a DriveItem via the API.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemReference {
    /// Identifier of the drive instance that contains the
    /// item. Read-only.
    pub drive_id: String,

    /// Identifies the type of drive. See drive resource for values.
    pub drive_type: String,

    /// Identifier of the item in the drive. Read-only.
    pub id: Option<String>,

    /// The name of the item being referenced. Read-only.
    pub name: Option<String>,

    /// Path that can be used to navigate to the item. Read-only.
    pub path: Option<String>,
}

impl File for DriveItem {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn modified_at(&self) -> i64 {
        if let Some(ts) = &self.last_modified_date_time {
            let dt = DateTime::<FixedOffset>::parse_from_rfc3339(ts);
            if let Ok(dt) = dt {
                return dt.timestamp();
            }
        }
        return 0;
    }

    fn created_at(&self) -> i64 {
        if let Some(ts) = &self.created_date_time {
            let dt = DateTime::<FixedOffset>::parse_from_rfc3339(ts);
            if let Ok(dt) = dt {
                return dt.timestamp();
            }
        }
        return 0;
    }

    fn is_folder(&self) -> bool {
        self.folder.is_some()
    }

    fn parent(&self) -> Option<String> {
        if let Some(parent) = &self.parent_reference {
            if let Some(id) = &parent.id {
                return Some(id.clone());
            }
        }
        return None;
    }
}

impl Delta for DriveItem {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn is_removed(&self) -> bool {
        self.deleted.is_some()
    }

    fn is_modified(&self) -> bool {
        !self.is_removed()
    }

    fn modified_at(&self) -> i64 {
        File::modified_at(self)
    }
}
