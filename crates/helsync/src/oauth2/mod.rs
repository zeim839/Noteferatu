//! OAuth2 authentication for cloud APIs.
//!
//! It exposes [Grant] and [Token], which abstract each step
//! in the PKCE authorization flow in a composable functional
//! style. Additionally, [PKCE] defines functions for generating and
//! verifying PKCE challenges.
//!
//! Relevant documentation:
//!  - [RFC6749: OAuth2 Framework](https://datatracker.ietf.org/doc/html/rfc6749)
//!  - [RFC7636: PKCE For OAuth2](https://datatracker.ietf.org/doc/html/rfc7636)
//!
//! # Examples
//! ## Full OAuth2 PKCE Authentication Flow
//! The example starts a local HTTP server and opens the user's
//! browser agent to the OAuth2 authorization URL. Once the user
//! authenticates, an authorization code is granted and the user's
//! browser agent is redirected to redirect_uri.
//!
//! Assuming the redirect_uri points to the local server, the user's
//! authorization grant is captured and is immediately exchanged for
//! an access token using [Grant::to_token].
//! ```no_run
//! use helsync::oauth2;
//!
//! #[tokio::main]
//! async fn main() {
//!
//!     // Define the OAuth2 application registration.
//!     let app_config = oauth2::Config::onedrive(
//!         "client-id", "http://localhost:6969"
//!     );
//!
//!     // Request authorization grant & exchange for access token.
//!     let token = app_config.from_grant_server(6969)
//!         .await.unwrap()
//!         .to_token()
//!         .await.unwrap();
//! }
//! ```
//!
//! ## Refreshing an Access Token
//! This example instantiates a [Token] by using a `refresh_token`
//! to get new access credentials.
//! ```no_run
//! use helsync::oauth2;
//!
//! #[tokio::main]
//! async fn main() {
//!
//!     // Define the OAuth2 application registration.
//!     let app_config = oauth2::Config::onedrive(
//!         "client-id", "http://localhost:6969"
//!     );
//!
//!     let mut token =
//!         oauth2::Token::from_refresh_token("my-refresh-token", &app_config)
//!         .await.unwrap();
//!
//!     if token.is_expired() {
//!         token.refresh(&app_config)
//!             .await.unwrap();
//!     }
//!
//!     // Or, equivalently:
//!     token.refresh_if_expired(&app_config)
//!         .await.unwrap();
//! }
//! ```
//!
//! ## Request Authorization Grant
//! oauth2 provides a [request_auth_grant] function for situations
//! where you need to request an authorization grant independent of a
//! [Grant] struct (or without a redirect server).
//!
//! This example will open the user's browser agent to the app's
//! OAuth2 authorization URL.
//! ```no_run
//! use helsync::oauth2;
//!
//! #[tokio::main]
//! async fn main() {
//!
//!     // Define the OAuth2 application registration.
//!     let app_config = oauth2::Config::onedrive(
//!         "client-id", "http://localhost:6969"
//!     );
//!
//!     // Create PKCE challenge.
//!     let pkce = oauth2::PKCE::new();
//!
//!     oauth2::request_auth_grant(&pkce, &app_config)
//!         .unwrap();
//! }
//! ```
mod config;
pub use config::*;

mod grant;
pub use grant::*;

mod pkce;
pub use pkce::*;

mod token;
pub use token::*;

mod utils;
pub use utils::*;
