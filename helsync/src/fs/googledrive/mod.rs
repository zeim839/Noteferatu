//! Google Drive API Client Filesystem.
//!
//! Official API Reference: [Google Drive API](https://developers.google.com/workspace/drive/api/guides/about-sdk)
//!
//! # Examples
//! ## Listing all Files
//! ```no_run
//! use helsync::fs::{FS, googledrive};
//! use helsync::oauth2;
//!
//! #[tokio::main]
//! async fn main() {
//!     let app_config = oauth2::Config::googledrive(
//!         "client-id", "client-secret", "http://localhost:6969",
//!     );
//!
//!     let token =
//!         oauth2::Token::from_refresh_token("example", &app_config)
//!         .await.unwrap();
//!
//!     let client = googledrive::GoogleDrive::new(&token, &app_config);
//!
//!     // Passing None selects the root directory. Alternatively,
//!     // pass a parent id: e.g. Some("parent-id").
//!     let files = client.list_files(None).await.unwrap();
//!
//!     for file in files {
//!         // Perform some element-wise operation...
//!     }
//! }
//! ```
//! ## Uploading a File
//! ```no_run
//! use helsync::fs::{FS, googledrive};
//! use helsync::oauth2;
//!
//! #[tokio::main]
//! async fn main() {
//!     let app_config = oauth2::Config::googledrive(
//!         "client-id", "client-secret", "http://localhost:6969",
//!     );
//!
//!     let token =
//!         oauth2::Token::from_refresh_token("example", &app_config)
//!         .await.unwrap();
//!
//!     let client = googledrive::GoogleDrive::new(&token, &app_config);
//!
//!     // Read file from local path.
//!     let bytes = std::fs::read("./some/local/file.txt").unwrap();
//!     let item = client.write_to_file(&bytes, None, "file.txt")
//!         .await.unwrap();
//!
//!     println!("Successfully uploaded item: {}", item.name);
//! }
//! ```
//! ## Downloading a File
//! ```no_run
//! use helsync::fs::{FS, googledrive};
//! use helsync::oauth2;
//!
//! #[tokio::main]
//! async fn main() {
//!     let app_config = oauth2::Config::googledrive(
//!         "client-id", "client-secret", "http://localhost:6969",
//!     );
//!
//!     let token =
//!         oauth2::Token::from_refresh_token("example", &app_config)
//!         .await.unwrap();
//!
//!     let client = googledrive::GoogleDrive::new(&token, &app_config);
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
//! use helsync::fs::{FS, googledrive};
//! use helsync::oauth2;
//!
//! #[tokio::main]
//! async fn main() {
//!     let app_config = oauth2::Config::googledrive(
//!         "client-id", "client-secret", "http://localhost:6969",
//!     );
//!
//!     let token =
//!         oauth2::Token::from_refresh_token("example", &app_config)
//!         .await.unwrap();
//!
//!     let client = googledrive::GoogleDrive::new(&token, &app_config);
//!
//!     // Fast forward to the current state (i.e. no changes).
//!     let (_, token) = client.track_changes(None, None).await.unwrap();
//!
//!     // NOTE: The changes API in Google Drive needs time to
//!     // propagate. It is a good idea to add delays.
//!
//!     // Add a delay to ensure token is committed.
//!     std::thread::sleep(std::time::Duration::from_secs(1));
//!
//!     // Perform some change (e.g. create a file).
//!     let buf = "Hello, World!".as_bytes();
//!     let file = client.write_to_file(buf, None, "my-new-file.txt")
//!         .await.unwrap();
//!
//!     // Add a delay to ensure change propagates..
//!     std::thread::sleep(std::time::Duration::from_secs(1));
//!
//!     // Fetch the change using the previously retrieved token.
//!     let (changes, _) = client.track_changes(None, Some(&token)).await.unwrap();
//!
//!     let find = changes.iter().find(|f| f.file_id == file.id);
//!     assert!(find.is_some());
//! }
//! ```

mod googledrive;
pub use googledrive::*;

mod file;
pub use file::*;

mod change;
pub use change::*;
