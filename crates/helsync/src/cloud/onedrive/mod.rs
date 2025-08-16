//! OneDrive [FileSystem](crate::core::FileSystem).
//!
//! Official API Reference: [Microsoft Graph](https://learn.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/?view=odsp-graph-online)
//! # Examples
//! ## Listing all Files
//! ```no_run
//! use helsync::{Filesystem, onedrive, oauth2};
//!
//! #[tokio::main]
//! async fn main() {
//!     let app_config = oauth2::Config::onedrive(
//!         "client-id", "http://localhost:6969"
//!     );
//!
//!     let token =
//!         oauth2::Token::from_refresh_token("example", &app_config)
//!         .await.unwrap();
//!
//!     let client = onedrive::OneDrive::new(None, &token, &app_config);
//!
//!     // Passing None selects the root directory. Alternatively,
//!     // pass a parent id: e.g. Some("parent-id").
//!     let files = client.list_files(None).await.unwrap();
//!
//!     for file in files {
//!         // Perform some file-wise operation...
//!     }
//! }
//! ```
//! ## Uploading a File
//! ```no_run
//! use helsync::{Filesystem, onedrive, oauth2};
//!
//! #[tokio::main]
//! async fn main() {
//!     let app_config = oauth2::Config::onedrive(
//!         "client-id", "http://localhost:6969"
//!     );
//!
//!     let token =
//!         oauth2::Token::from_refresh_token("example", &app_config)
//!         .await.unwrap();
//!
//!     let client = onedrive::OneDrive::new(None, &token, &app_config);
//!
//!     // First, create a new file to get an ID.
//!     let file = client.create_file(None, "file.txt")
//!         .await.unwrap();
//!
//!     // Then, write the content to it.
//!     let bytes = std::fs::read("./some/local/file.txt").unwrap();
//!     let item = client.write_to_file(&file.id, &bytes)
//!         .await.unwrap();
//!
//!     println!("Successfully uploaded item: {}", item.name.unwrap());
//! }
//! ```
//! ## Downloading a File
//! ```no_run
//! use helsync::{Filesystem, onedrive, oauth2};
//!
//! #[tokio::main]
//! async fn main() {
//!     let app_config = oauth2::Config::onedrive(
//!         "client-id", "http://localhost:6969"
//!     );
//!
//!     let token =
//!         oauth2::Token::from_refresh_token("example", &app_config)
//!         .await.unwrap();
//!
//!     let client = onedrive::OneDrive::new(None, &token, &app_config);
//!
//!     // Read a file with the given file id.
//!     let data = client.read_from_file("some-file-id").await.unwrap();
//!
//!     // Print bytes to console.
//!     let string = String::from_utf8(data.clone()).unwrap();
//!     println!("{string}");
//!
//!     // Save bytes to a file.
//!     std::fs::write("my-file-name.txt", data).unwrap();
//! }
//! ```
//! ## Tracking Changes
//! ```no_run
//! use helsync::{Filesystem, onedrive, oauth2};
//!
//! #[tokio::main]
//! async fn main() {
//!     let app_config = oauth2::Config::onedrive(
//!         "client-id", "http://localhost:6969"
//!     );
//!
//!     let token =
//!         oauth2::Token::from_refresh_token("example", &app_config)
//!         .await.unwrap();
//!
//!     let client = onedrive::OneDrive::new(None, &token, &app_config);
//!
//!     // Use token "latest" to fast forward to latest changes.
//!     let (_, delta) = client.track_changes(Some("latest"))
//!         .await.unwrap();
//!
//!     // Perform some change (e.g. create and write to a file).
//!     let file = client.create_file(None, "my-new-file.txt")
//!         .await.unwrap();
//!     let buf = "Hello, World!".as_bytes();
//!     client.write_to_file(&file.id, buf).await.unwrap();
//!
//!     // Fetch changes using latest delta token.
//!     let (changes, _) = client.track_changes(Some(delta.as_str()))
//!         .await.unwrap();
//!
//!     // Search for the newly uploaded file in the changes.
//!     let item = changes.iter().find(|f| f.id == file.id);
//!
//!     // File should exist and should have FileMetadata.
//!     assert!(item.is_some());
//!     assert!(item.unwrap().file.is_some());
//! }
//! ```
mod client;
pub use client::*;

mod error;
pub use error::*;

mod file;
pub use file::*;

mod status;
pub use status::*;
