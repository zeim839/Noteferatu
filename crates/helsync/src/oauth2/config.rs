use super::utils::*;
use super::grant::Grant;
use crate::core::Result;

/// An OAuth2 app configuration.
///
/// Note that [Config::client_secret] is optionally included to
/// support Google Drive clients. The `client_secret` is required by
/// the Google token API route, even if PKCE authentication is used.
/// Nevertheless, it is safe to store a `client_secret` in this case,
/// even in a public client.
///
/// See: [StackOverFlow](https://stackoverflow.com/questions/60724690/using-google-oidc-with-code-flow-and-pkce)
#[derive(Clone)]
pub struct Config {
    pub auth_endpoint: String,
    pub token_endpoint: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub redirect_uri: String,
    pub scope: String,
}

impl Config {

    /// Constructs an [Config] for a OneDrive API client.
    pub fn onedrive(client_id: &str, redirect_uri: &str) -> Self {
        Self {
            auth_endpoint: auth_endpoint("OneDrive"),
            token_endpoint: token_endpoint("OneDrive"),
            client_id: client_id.to_string(),
            client_secret: None,
            redirect_uri: redirect_uri.to_string(),
            scope: scope("OneDrive"),
        }
    }

    /// Constructs an [Config] for a Google Drive API client.
    pub fn googledrive(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        Self {
            auth_endpoint: auth_endpoint("GoogleDrive"),
            token_endpoint: token_endpoint("GoogleDrive"),
            client_id: client_id.to_string(),
            client_secret: Some(client_secret.to_string()),
            redirect_uri: redirect_uri.to_string(),
            scope: scope("GoogleDrive"),
        }
    }

    /// Obtain an OAuth2 authorization [grant](Grant) by requesting
    /// the user's authorization via browser agent and capture an
    /// authorization grant redirect using a local HTTP server that
    /// listen in the background.
    pub async fn from_grant_server(&self, port: u16) -> Result<Grant> {
        Grant::from_server(port, self).await
    }

    /// Instantiate an OAuth2 authorization [grant](Grant) from a
    /// known code.
    pub fn from_grant_code(&self, code: &str) -> Grant {
        Grant::from_code(code, self)
    }
}
