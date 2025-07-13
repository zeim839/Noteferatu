use crate::fs::{FS, client::Client};
use crate::fs::utils::extract_query_params;
use crate::oauth2::{Config, Token};

use super::status::{JobStatus, StatusReport};
use super::item::DriveItem;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use anyhow::{Result, anyhow};
use serde_json::{Value, from_value};
use std::sync::Arc;

/// Microsoft Graph API endpoint for OneDrive.
pub const API_ENDPOINT: &str = "https://graph.microsoft.com/v1.0/me/drive";

/// Implements a OneDrive API client.
pub struct OneDrive {
    client: Client,
    req: Arc<reqwest::Client>,
}

impl OneDrive {

    /// Instantiate new OneDrive client.
    pub fn new(token: &Token, config: &Config) -> Self {
        Self {
            client: Client::new(token, config),
            req: Arc::new(reqwest::Client::new()),
        }
    }

    /// Upload small files (< 4MB) directly.
    async fn upload_small_file(&self, buf: &[u8], parent_id: Option<&str>, name: &str) -> Result<DriveItem> {
        let url = match parent_id {
            Some(p) => format!("{}/items/{}:/{}:/content", API_ENDPOINT, p, name),
            None => format!("{}/root:/{}:/content", API_ENDPOINT, name),
        };

        let req = self.req.clone().put(&url)
            .header(AUTHORIZATION, self.client.bearer().await?)
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(buf.to_vec());

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let json: Value = res.json().await?;
        let item: DriveItem = from_value(json)?;

        Ok(item)
    }

    /// Upload large files (>= 4MB) using upload session.
    async fn upload_large_file(&self, buf: &[u8], parent_id: Option<&str>, name: &str) -> Result<DriveItem> {

        // Create upload session.
        let session_url = match parent_id {
            Some(p) => format!("{}/items/{}:/{}:/createUploadSession", API_ENDPOINT, p, name),
            None => format!("{}/root:/{}:/createUploadSession", API_ENDPOINT, name),
        };

        let session_body = serde_json::json!({
            "item": {
                "@microsoft.graph.conflictBehavior": "replace"
            }
        });

        let req = self.req.clone().post(&session_url)
            .header(AUTHORIZATION, self.client.bearer().await?)
            .json(&session_body);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let session_json: Value = res.json().await?;
        let upload_url = session_json["uploadUrl"]
            .as_str()
            .ok_or(anyhow!("no uploadUrl in session response"))?;

        // Upload file in chunks.
        let chunk_size = 320 * 1024; // 320 KB chunks.
        let total_size = buf.len();
        let mut offset = 0;

        while offset < total_size {
            let end = std::cmp::min(offset + chunk_size, total_size);
            let chunk = &buf[offset..end];

            let req = self.req.clone().put(upload_url)
                .header("Content-Range", format!("bytes {}-{}/{}", offset, end - 1, total_size))
                .header("Content-Length", chunk.len())
                .body(chunk.to_vec());

            let res = self.client.execute_with_retry(req).await?
                .error_for_status()?;

            // Check if upload is complete.
            if res.status() == 201 {
                let json: Value = res.json().await?;
                let item: DriveItem = from_value(json)?;
                return Ok(item);
            }

            offset = end;
        }

        Err(anyhow!("upload completed but no response received"))
    }
}

impl FS for OneDrive {

    type File = DriveItem;
    type Delta = DriveItem;

    /// Retrieve the metadata for a [DriveItem] in a Drive by id.
    ///
    /// API Reference: [Get Item](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get?view=odsp-graph-online)
    async fn get_file(&self, source_id: &str) -> Result<DriveItem> {
        let url = format!("{}/items/{}", API_ENDPOINT, source_id);
        let req = self.req.clone().get(&url)
            .header(AUTHORIZATION, self.client.bearer().await?);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let json: Value = res.json().await?;
        let item: DriveItem = from_value(json)?;

        Ok(item)
    }

    /// Copy a [DriveItem].
    ///
    /// Copies the file with id `source_id` to the parent
    /// `parent_id`. If `parent_id` is `None`, the file is copied to
    /// the root directory. Optionally specifying a `name` will rename
    /// the copied item.
    ///
    /// API Reference: [Copy a DriveItem](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_copy?view=odsp-graph-online)
    async fn copy_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) -> Result<DriveItem> {
        let url = format!("{}/items/{}/copy", API_ENDPOINT, source_id);
        let mut body = serde_json::json!({});
        if let Some(parent_id) = parent_id {
            body = serde_json::json!({
                "parentReference": {
                    "id": parent_id,
                }
            });
        }

        if let Some(name) = name {
            body["name"] = serde_json::Value::String(name.to_string());
        }

        // Post the copy request.
        let req = self.req.clone().post(&url)
            .header(AUTHORIZATION, self.client.bearer().await?)
            .json(&body);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        // Extract the location header to monitor status.
        let location_header = res.headers().get("location")
            .ok_or(anyhow!("request did not respond with location header"))?;

        let location = location_header.to_str()?;
        let res = self.req.clone().get(location).send()
            .await?.error_for_status()?;

        // Monitor the operation's status.
        let json: Value = res.json().await?;
        let mut status: JobStatus = from_value(json)?;
        while status.status != StatusReport::Completed {
            match status.status {
                StatusReport::Failed |
                StatusReport::CancelPending |
                StatusReport::Cancelled => {
                    return Err(anyhow!("operation was not completed"));
                },
                StatusReport::Completed => break,
                _ => {
                    let res = self.req.clone().get(location).send()
                        .await?.error_for_status()?;

                    let json: Value = res.json().await?;
                    status = from_value(json)?;
                }
            }
        }

        // Use the resource_id to fetch the copied item.
        Ok(self.get_file(&status.resource_id).await?)
    }

    /// Move a [DriveItem] to a new folder.
    ///
    /// Setting `parent_id` to `None` moves the child to the root
    /// directory. Specifying a `name` renames the child.
    ///
    /// API Reference: [Move](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_move?view=odsp-graph-online)
    async fn move_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) -> Result<DriveItem> {
        let url = format!("{}/items/{}", API_ENDPOINT, source_id);
        let mut body = serde_json::json!({
            "parentReference": {
                "path": "/drive/root",
            }
        });

        if let Some(parent_id) = parent_id {
            body = serde_json::json!({
                "parentReference": {
                    "id": parent_id,
                }
            });
        }

        if let Some(new_name) = name {
            body["name"] = serde_json::Value::String(new_name.to_string());
        }

        let req = self.req.clone().patch(&url)
            .header(AUTHORIZATION, self.client.bearer().await?)
            .json(&body);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let json: Value = res.json().await?;
        let item: DriveItem = from_value(json)?;

        Ok(item)
    }

    /// Delete a [DriveItem].
    ///
    /// Note that deleting items using this method will move the items
    /// to the recycle bin instead of permanently deleting the item.
    ///
    /// API Reference: [Delete](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_delete?view=odsp-graph-online)
    async fn remove_file(&self, id: &str) -> Result<()> {
        let url = format!("{}/items/{}", API_ENDPOINT, id);
        let req = self.req.clone().delete(&url)
            .header(AUTHORIZATION, self.client.bearer().await?);

        self.client.execute_with_retry(req).await?
            .error_for_status()?;

        Ok(())
    }

    /// Create a new folder.
    ///
    /// Create a new folder or with a under the parent `parent_id`. If
    /// `parent_id` is `None`, the folder is created in the root
    /// directory.
    ///
    /// API Reference: [Create folder](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_post_children?view=odsp-graph-online)
    async fn create_folder(&self, parent_id: Option<&str>, name: &str) -> Result<DriveItem> {
        let url = match parent_id {
            Some(p) => format!("{}/items/{}/children", API_ENDPOINT, p),
            None => format!("{}/root/children", API_ENDPOINT),
        };

        let items = serde_json::json!({
            "name": name,
            "folder": {},
            "@microsoft.graph.conflictBehavior": "fail",
        });

        let req = self.req.clone().post(&url)
            .header(AUTHORIZATION, self.client.bearer().await?)
            .json(&items);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let json: Value = res.json().await?;
        let item: DriveItem = from_value(json)?;

        Ok(item)
    }

    /// List the children of a directory.
    ///
    /// Return a collection of [DriveItems](DriveItem) in the children
    /// relationship of a [DriveItem]. Set `id` to `None` to retrieve
    /// the children of the root directory.
    ///
    /// DriveItems with a non-null folder or package facet can have
    /// one or more child DriveItems.
    ///
    /// API Reference: [List Children](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_list_children?view=odsp-graph-online)
    async fn list_files(&self, id: Option<&str>) -> Result<Vec<DriveItem>> {
        let url = match id {
            Some(p) => format!("{}/items/{}/children", API_ENDPOINT, p),
            None => format!("{}/root/children", API_ENDPOINT),
        };

        let req = self.req.clone().get(&url)
            .header(AUTHORIZATION, self.client.bearer().await?);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let json: Value = res.json().await?;
        let items: Vec<DriveItem> = from_value(json["value"].clone())?;

        Ok(items)
    }

    /// Track changes for a Drive.
    ///
    /// Setting `id` to `None` returns the changes relative to the
    /// root directory. `delta` specifies a token for fast-forwarding
    /// to the latest changes.
    ///
    /// API Reference: [Sync Changes](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_delta?view=odsp-graph-online)
    async fn track_changes(&self, id: Option<&str>, delta: Option<&str>) -> Result<(Vec<DriveItem>, String)> {

        // Paginated endpoint: initial request.
        let mut changes: Vec<DriveItem> = Vec::new();
        let url = match id {
            Some(p) => format!("{}/items/{}/delta", API_ENDPOINT, p),
            None => format!("{}/root/delta", API_ENDPOINT),
        };

        // Applying a delta omits changes that have already been viewed.
        let url = match delta {
            Some(p) => format!("{}?token={}", url, p),
            None => url,
        };

        let req = self.req.clone().get(&url)
            .header(AUTHORIZATION, self.client.bearer().await?);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let mut json: Value = res.json().await?;
        let mut items: Vec<DriveItem> = from_value(json["value"].clone())?;
        changes.append(&mut items);

        // Subsequent pages.
        while let Some(url) = json.get("@odata.nextLink") {
            let url_str = url.as_str()
                .ok_or(anyhow!("invalid URL in @odata.nextLink"))?;

            let req = self.req.clone().get(url_str)
                .header(AUTHORIZATION, self.client.bearer().await?);

            let res = self.client.execute_with_retry(req).await?
                .error_for_status()?;

            json = res.json().await?;
            let mut items: Vec<DriveItem> = from_value(json["value"].clone())?;
            changes.append(&mut items);
        }

        let next_delta = json["@odata.deltaLink"].as_str().unwrap_or_default();
        let next_delta = extract_query_params(next_delta)
            .get("token")
            .cloned()
            .unwrap_or_default();

        Ok((changes, next_delta))
    }

    /// Upload or replace the contents of a [DriveItem].
    ///
    /// Passing `None` to `parent_id` sets the root directory as the
    /// parent. Use `name` to change the file's name (uses the local
    /// file name by default).
    ///
    /// API Reference: [Upload](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_createuploadsession?view=odsp-graph-online)
    async fn write_to_file(&self, buf: &[u8], parent_id: Option<&str>, name: &str) -> Result<DriveItem> {
        if buf.len() <= 4 * 1024 * 1024 {
            self.upload_small_file(buf, parent_id, name).await
        } else {
            self.upload_large_file(buf, parent_id, name).await
        }
    }

    /// Download the contents of the primary stream (file) of a
    /// [DriveItem]. Only driveItems with the file property can be
    /// downloaded.
    ///
    /// API Reference: [Download](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get_content?view=odsp-graph-online)
    async fn read_from_file(&self, id: &str) -> Result<Vec<u8>> {
        let url = format!("{}/items/{}/content", API_ENDPOINT, id);
        let req = self.req.clone().get(&url)
            .header(AUTHORIZATION, self.client.bearer().await?);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let bytes = res.bytes().await?;
        Ok(bytes.to_vec())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::oauth2;
    use std::env;
    use dotenv::dotenv;

    use std::sync::Arc;
    use tokio::sync::OnceCell;
    static TOKEN: OnceCell<Arc<oauth2::Token>> = OnceCell::const_new();

    async fn get_test_client() -> OneDrive {
        dotenv().ok();

        let client_id = env::var("ONEDRIVE_CLIENT_ID")
            .expect("missing ONEDRIVE_CLIENT_ID env variable");

        let redirect_uri = env::var("ONEDRIVE_REDIRECT_URI")
            .expect("missing ONEDRIVE_REDIRECT_URI env variable");

        let refresh_token = env::var("ONEDRIVE_REFRESH_TOKEN")
            .expect("missing ONEDRIVE_REFRESH_TOKEN env variable");

        let app_config = oauth2::Config::onedrive(&client_id, &redirect_uri);

        let token = TOKEN.get_or_init(|| async {
            let token = oauth2::Token::from_refresh_token(&refresh_token, &app_config).await.unwrap();
            Arc::new(token)
        }).await.clone();

        OneDrive::new(&token, &app_config)
    }

    #[tokio::test]
    async fn test_get_file() {
        let client = get_test_client().await;
        let buf = "Hello, World!".as_bytes();
        let file = client.write_to_file(buf, None, "helsync-get-file.txt")
            .await.unwrap();

        let res = client.get_file(&file.id).await.unwrap();
        assert!(res.id == file.id);
        client.remove_file(&file.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_copy_file() {
        let client = get_test_client().await;
        let buf = "Hello, World!".as_bytes();
        let file = client.write_to_file(buf, None, "helsync-copy-file.txt")
            .await.unwrap();

        let copied = client.copy_file(&file.id, None, Some("new-name.txt"))
            .await.unwrap();

        assert!(copied.id != file.id);
        assert!(copied.name.unwrap() == "new-name.txt");

        let copied_verify = client.get_file(&copied.id).await.unwrap();
        assert!(copied_verify.id == copied.id);
        assert!(copied_verify.name.unwrap() == "new-name.txt");

        client.remove_file(&copied_verify.id).await.unwrap();
        client.remove_file(&file.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_move_file() {
        let client = get_test_client().await;
        let buf = "Hello, World!".as_bytes();
        let child = client.write_to_file(buf, None, "helsync-child-file.txt")
            .await.unwrap();

        let parent = client.create_folder(None, "helsync-parent")
            .await.unwrap();

        let moved = client.move_file(&child.id, Some(&parent.id), None)
            .await.unwrap();

        assert!(moved.parent_reference.unwrap().name.unwrap() == "helsync-parent");
        client.remove_file(&parent.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_folder() {
        let client = get_test_client().await;
        let parent = client.create_folder(None, "helsync-test-folder")
            .await.unwrap();

        assert!(parent.name.unwrap() == "helsync-test-folder");
        assert!(parent.folder.is_some());
        assert!(parent.file.is_none());

        client.remove_file(&parent.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_list_files() {
        let client = get_test_client().await;
        let parent = client.create_folder(None, "helsync-test-list")
            .await.unwrap();

        let files = client.list_files(None).await.unwrap();
        let find = files.iter().find(|f| f.id == parent.id);
        assert!(find.is_some());

        client.remove_file(&parent.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_track_changes() {
        let client = get_test_client().await;
        let (_, token) = client.track_changes(None, Some("latest")).await.unwrap();

        // Make a change.
        let buf = "Hello, World!".as_bytes();
        let file = client.write_to_file(buf, None, "helsync-track-changes.txt")
            .await.unwrap();

        let (changes, _) = client.track_changes(None, Some(&token)).await.unwrap();
        let find = changes.iter().find(|f| f.id == file.id);
        assert!(find.is_some());

        client.remove_file(&file.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_read_write_small() {
        let client = get_test_client().await;
        let buf = "Hello, World!".as_bytes();
        let file = client.write_to_file(buf, None, "helsync-write-small.txt")
            .await.unwrap();

        assert!(file.name.unwrap() == "helsync-write-small.txt");
        let v = client.read_from_file(&file.id).await.unwrap();
        let string = String::from_utf8(v).unwrap();
        assert!(string == "Hello, World!");

        client.remove_file(&file.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_read_write_large() {
        let client = get_test_client().await;

        // 5MB of 'A' characters.
        let file_size = 5 * 1024 * 1024;
        let buf = vec![b'A'; file_size];

        let file = client.write_to_file(&buf, None, "helsync-write-large.txt")
            .await.unwrap();

        assert!(file.name.unwrap() == "helsync-write-large.txt");
        client.remove_file(&file.id).await.unwrap();
    }
}
