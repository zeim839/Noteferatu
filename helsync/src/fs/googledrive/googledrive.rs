use crate::fs::{FS, Delta, client::Client};
use crate::oauth2::{Config, Token};
use super::file::DriveFile;
use super::change::DriveChange;

use reqwest::header::AUTHORIZATION;
use form_urlencoded::byte_serialize;
use anyhow::{Result, anyhow};
use serde_json::{Value, from_value};
use std::sync::Arc;

/// Google API endpoint for Google Drive.
pub const API_ENDPOINT: &str = "https://www.googleapis.com/drive/v3";

/// Implements a Google Drive API client.
pub struct GoogleDrive {
    client: Client,
    req: Arc<reqwest::Client>,
}

impl GoogleDrive {

    /// Instantiate new Google Drive client.
    pub fn new(token: &Token, config: &Config) -> Self {
        Self {
            client: Client::new(token, config),
            req: Arc::new(reqwest::Client::new()),
        }
    }

    /// Upload large files (>= 5MB) using resumable upload
    async fn upload_large_file(&self, buf: &[u8], parent_id: Option<&str>, name: &str) -> Result<DriveFile> {
        let mut metadata = serde_json::json!({
            "name": name
        });

        if let Some(parent) = parent_id {
            metadata["parents"] = serde_json::json!([parent]);
        }

        // Start resumable upload session.
        let session_url = "https://www.googleapis.com/upload/drive/v3/files?uploadType=resumable";

        let req = self.req.clone().post(session_url)
            .header(AUTHORIZATION, self.client.bearer().await?)
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("X-Upload-Content-Length", buf.len().to_string())
            .json(&metadata);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let upload_url = res.headers().get("location")
            .and_then(|h| h.to_str().ok())
            .ok_or(anyhow!(
                "No location header in resumable upload response"
            ))?;

        // Upload in chunks
        let chunk_size = 256 * 1024; // 256KB chunks.
        let total_size = buf.len();
        let mut offset = 0;

        while offset < total_size {
            let end = std::cmp::min(offset + chunk_size, total_size);
            let chunk = &buf[offset..end];

            let req = self.req.clone().put(upload_url)
                .header(AUTHORIZATION, self.client.bearer().await?)
                .header("Content-Range", format!("bytes {}--{}/{}", offset, end - 1, total_size))
                .header("Content-Length", chunk.len().to_string())
                .body(chunk.to_vec());

            let res = self.client.execute_with_retry(req).await?
                .error_for_status()?;

            // Check if upload is complete.
            if end >= total_size {
                let json: Value = res.json().await?;
                let item: DriveFile = from_value(json)?;
                return Ok(item);
            }

            offset = end;
        }

        Err(anyhow!("upload completed but no response received"))
    }

    /// Upload small files (< 5MB) directly
    async fn upload_small_file(&self, buf: &[u8], parent_id: Option<&str>, name: &str) -> Result<DriveFile> {
        let mut metadata = serde_json::json!({
            "name": name
        });

        if let Some(parent) = parent_id {
            metadata["parents"] = serde_json::json!([parent]);
        }

        let url = "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart";

        // Create multipart body
        let boundary = "boundary123456789";
        let mut body = Vec::new();

        // Metadata part
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(b"Content-Type: application/json; charset=UTF-8\r\n\r\n");
        body.extend_from_slice(metadata.to_string().as_bytes());
        body.extend_from_slice(b"\r\n");

        // File content part
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(b"Content-Type: application/octet-stream\r\n\r\n");
        body.extend_from_slice(&buf);
        body.extend_from_slice(format!("\r\n--{}--\r\n", boundary).as_bytes());

        let req = self.req.clone().post(url)
            .header(AUTHORIZATION, self.client.bearer().await?)
            .header("Content-Type", format!("multipart/related; boundary={}", boundary))
            .body(body);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let json: Value = res.json().await?;
        let file: DriveFile = from_value(json)?;

        Ok(file)
    }
}

impl FS for GoogleDrive {

    type File = DriveFile;
    type Delta = DriveChange;

    /// Gets a file's metadata or content by ID.
    ///
    /// API Reference: [Get](https://developers.google.com/workspace/drive/api/reference/rest/v3/files/get)
    async fn get_file(&self, id: &str) -> Result<DriveFile> {
        let url = format!("{}/files/{}", API_ENDPOINT, id);
        let req = self.req.clone().get(&url)
            .header(AUTHORIZATION, self.client.bearer().await?);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let json: Value = res.json().await?;
        let file: DriveFile = from_value(json)?;

        Ok(file)
    }

    /// Creates a copy of a file and applies any requested updates
    /// with patch semantics.
    ///
    /// API Reference: [Copy](https://developers.google.com/workspace/drive/api/reference/rest/v3/files/copy)
    async fn copy_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) -> Result<DriveFile> {
        let url = format!("{}/files/{}/copy", API_ENDPOINT, source_id);
        let mut body = serde_json::json!({});
        if let Some(new_name) = name {
            body["name"] = serde_json::Value::String(new_name.to_string());
        }

        if let Some(parent) = parent_id {
            body["parents"] = serde_json::json!([parent]);
        }

        let req = self.req.clone().post(&url)
            .header(AUTHORIZATION, self.client.bearer().await?)
            .json(&body);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let json: Value = res.json().await?;
        let file: DriveFile = from_value(json)?;

        Ok(file)
    }

    /// Updates the file metadata to move the file under a new parent.
    ///
    /// API Reference: [Update](https://developers.google.com/workspace/drive/api/reference/rest/v3/files/update)
    async fn move_file(&self, source_id: &str, parent_id: Option<&str>, name: Option<&str>) -> Result<DriveFile> {
        let current_file = self.get_file(source_id).await?;
        let mut url = format!("{}/files/{}", API_ENDPOINT, source_id);
        let mut params = Vec::new();

        if let Some(parent_id) = parent_id {
            if let Some(current_parents) = &current_file.parents {
                let remove_parents = current_parents.join(",");
                params.push(format!("removeParents={}", remove_parents));
            }
            params.push(format!("addParents={}", parent_id));
        }

        // Add fields parameter to ensure we get back the parents field
        params.push("fields=id,name,parents,mimeType,size,createdTime,modifiedTime,trashed,version".to_string());

        if !params.is_empty() {
            url.push_str("?");
            url.push_str(&params.join("&"));
        }

        let mut body = serde_json::json!({});
        if let Some(new_name) = name {
            body["name"] = serde_json::Value::String(new_name.to_string());
        }

        let req = self.req.clone().patch(&url)
            .header(AUTHORIZATION, self.client.bearer().await?)
            .json(&body);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let json: Value = res.json().await?;
        let file: DriveFile = from_value(json)?;

        Ok(file)
    }

    /// Permanently deletes a file owned by the user without moving it
    /// to the trash. If the target is a folder, all descendants owned
    /// by the user are also deleted.
    ///
    /// API Reference: [Delete](https://developers.google.com/workspace/drive/api/reference/rest/v3/files/delete)
    async fn remove_file(&self, id: &str) -> Result<()> {
        let url = format!("{}/files/{}", API_ENDPOINT, id);
        let req = self.req.clone().delete(&url)
            .header(AUTHORIZATION, self.client.bearer().await?);

        self.client.execute_with_retry(req).await?
            .error_for_status()?;

        Ok(())
    }

    /// Creates a new folder.
    ///
    /// API Reference: [Create](https://developers.google.com/workspace/drive/api/reference/rest/v3/files/create)
    async fn create_folder(&self, parent_id: Option<&str>, name: &str) -> Result<DriveFile> {
        let body = serde_json::json!({
            "name": name,
            "parents": parent_id.map(|p| vec![p.to_string()]),
            "mimeType": "application/vnd.google-apps.folder",
        });

        let req = self.req.clone().post(format!("{}/files", API_ENDPOINT))
            .header(AUTHORIZATION, self.client.bearer().await?)
            .json(&body);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let drive_file: DriveFile = res.json().await?;
        Ok(drive_file)
    }

    /// Lists the user's files.
    ///
    /// API Reference: [List](https://developers.google.com/workspace/drive/api/reference/rest/v3/files/list)
    async fn list_files(&self, parent_id: Option<&str>) -> Result<Vec<DriveFile>> {
        let mut url = format!("{API_ENDPOINT}/files");
        if let Some(p) = parent_id {
            let q = format!("parents in '{p}'");
            url.push_str(&format!("?q={}", byte_serialize(q.as_bytes()).collect::<String>()))
        }

        let req = self.req.clone().get(&url)
            .header(AUTHORIZATION, self.client.bearer().await?);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let json: Value = res.json().await?;
        let files: Vec<DriveFile> =
            from_value(json["files"].clone())?;

        Ok(files)
    }

    /// Report file changes.
    async fn track_changes(&self, _: Option<&str>, token: Option<&str>) -> Result<(Vec<DriveChange>, String)> {
        let mut url = format!(
            "{}/changes?{}", API_ENDPOINT,
            form_urlencoded::Serializer::new(String::new())
                .append_pair("includeRemoved", "true")
                .append_pair("restrictToMyDrive", "true")
                .append_pair("pageSize", "20")
                .finish()
        );

        if let Some(token) = token {
            url.push_str(&format!("&pageToken={token}"));
        } else {
            let start_token_url =
                format!("{API_ENDPOINT}/changes/startPageToken");

            let req = self.req.clone().get(start_token_url)
                .header(AUTHORIZATION, self.client.bearer().await?);

            let res = self.client.execute_with_retry(req).await?
                .error_for_status()?;

            let json: Value = res.json().await?;
            let start_token = json["startPageToken"]
                .as_str()
                .ok_or(anyhow!("not startPageToken in response"))?;

            url.push_str(&format!("&pageToken={}", start_token));
        }

        let mut changes: Vec<DriveChange> = Vec::new();
        let req = self.req.clone().get(&url)
            .header(AUTHORIZATION, self.client.bearer().await?);

        let res = self.client.execute_with_retry(req).await?
            .error_for_status()?;

        let mut json: Value = res.json().await?;
        let mut items: Vec<DriveChange> =
            from_value(json["changes"].clone())?;

        changes.append(&mut items);

        // Handle pagination.
        while let Some(next_token) = json.get("nextPageToken").and_then(|t| t.as_str()) {
            let next_url = format!(
                "{}/changes?{}", API_ENDPOINT,
                form_urlencoded::Serializer::new(String::new())
                    .append_pair("includeRemoved", "true")
                    .append_pair("restrictToMyDrive", "true")
                    .append_pair("pageToken", next_token)
                    .append_pair("pageSize", "20")
                    .finish()
            );

            let req = self.req.clone().get(&next_url)
                .header(AUTHORIZATION, self.client.bearer().await?);

            let res = self.client.execute_with_retry(req).await?
                .error_for_status()?;

            json = res.json().await?;
            let mut items: Vec<DriveChange> =
                from_value(json["changes"].clone())?;

            changes.append(&mut items);
        }

        let next_page_token = json
            .get("newStartPageToken")
            .and_then(|t| t.as_str())
            .unwrap_or_default()
            .to_string();

        Ok((changes, next_page_token))
    }

    /// Updates the contents of a file, otherwise creating it if it
    /// doesn't exist.
    ///
    /// This method supports an /upload URI and accepts uploaded
    /// media.
    ///
    /// Documentation: [Upload File Data](https://developers.google.com/workspace/drive/api/guides/manage-uploads)
    /// API Reference: [Create](https://developers.google.com/workspace/drive/api/reference/rest/v3/files/create)
    async fn write_to_file(&self, buf: &[u8], parent_id: Option<&str>, name: &str) -> Result<DriveFile> {
        if buf.len() > 5 * 1024 * 1024 {
            self.upload_large_file(buf, parent_id, name).await
        } else {
            self.upload_small_file(buf, parent_id, name).await
        }
    }

    /// Downloads content of a file. Operations are valid for 24 hours
    /// from the time of creation.
    ///
    /// API Reference: [Download](https://developers.google.com/workspace/drive/api/reference/rest/v3/files/download)
    async fn read_from_file(&self, id: &str) -> Result<Vec<u8>> {
        let url = format!("{}/files/{}?alt=media", API_ENDPOINT, id);
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

    async fn get_test_client() -> GoogleDrive {
        dotenv().ok();

        let client_id = env::var("GOOGLEDRIVE_CLIENT_ID")
            .map_err(|_| anyhow!("missing GOOGLEDRIVE_CLIENT_ID env variable"))
            .unwrap();

        let client_secret = env::var("GOOGLEDRIVE_CLIENT_SECRET")
            .map_err(|_| anyhow!("missing GOOGLEDRIVE_CLIENT_SECRET env variable"))
            .unwrap();

        let redirect_uri = env::var("GOOGLEDRIVE_REDIRECT_URI")
            .map_err(|_| anyhow!("missing GOOGLEDRIVE_CLIENT_SECRET env variable"))
            .unwrap();

        let refresh_token = env::var("GOOGLEDRIVE_REFRESH_TOKEN")
            .map_err(|_| anyhow!("missing GOOGLEDRIVE_REFRESH_TOKEN env variable"))
            .unwrap();

        let app_config = oauth2::Config::googledrive(
            &client_id, &client_secret, &redirect_uri,
        );

        let token = TOKEN.get_or_init(|| async {
            let token = oauth2::Token::from_refresh_token(&refresh_token, &app_config).await.unwrap();
            Arc::new(token)
        }).await.clone();

        GoogleDrive::new(&token, &app_config)
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
        assert!(copied.name == "new-name.txt");

        let copied_verify = client.get_file(&copied.id).await.unwrap();
        assert!(copied_verify.id == copied.id);
        assert!(copied_verify.name == "new-name.txt");

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

        assert!(moved.parents.unwrap().get(0).unwrap() == &parent.id);
        client.remove_file(&parent.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_folder() {
        let client = get_test_client().await;
        let parent = client.create_folder(None, "helsync-test-folder")
            .await.unwrap();

        assert!(parent.name == "helsync-test-folder");
        assert!(parent.mime_type.unwrap() == "application/vnd.google-apps.folder");
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
        let (_, token) = client.track_changes(None, None).await.unwrap();

        // Changes propagate slowly, especially when a lot of tests
        // are simultaneously committing changes. Adding significant
        // delays allows the changes to accumulate.

        // Add a small delay to ensure the token is "committed".
        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;

        // Make a change.
        let buf = "Hello, World!".as_bytes();
        let file = client.write_to_file(buf, None, "helsync-track-changes.txt")
            .await.unwrap();

        // Wait a bit for the change to propagate.
        tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;

        let (changes, _) = client.track_changes(None, Some(&token)).await.unwrap();
        let find = changes.iter().find(|f| f.id() == file.id);
        assert!(find.is_some());

        client.remove_file(&file.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_read_write_small() {
        let client = get_test_client().await;
        let buf = "Hello, World!".as_bytes();
        let file = client.write_to_file(buf, None, "helsync-write-small.txt")
            .await.unwrap();

        assert!(file.name == "helsync-write-small.txt");
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

        assert!(file.name == "helsync-write-large.txt");
        client.remove_file(&file.id).await.unwrap();
    }
}
