// -- TODO --
// Are the provider functions still needed? They should at least use a
// cloud provider enumeration instead of &str.
// -----------
use anyhow::Result;

/// Get the OAuth2 authorization endpoint for the given cloud
/// provider. Currently supported options are "OneDrive" and
/// "GoogleDrive".
pub fn auth_endpoint(provider: &str) -> Result<String> {
    match provider {
        "OneDrive" => Ok(ONEDRIVE_AUTH_ENDPOINT.to_string()),
        "GoogleDrive" => Ok(GOOGLEDRIVE_AUTH_ENDPOINT.to_string()),
        _ => Err(anyhow::anyhow!("unsupported provider \"{provider}\"")),
    }
}

/// Get the OAuth2 token endpoint for the given cloud
/// provider. Currently supported options are "OneDrive" and
/// "GoogleDrive".
pub fn token_endpoint(provider: &str) -> Result<String> {
    match provider {
        "OneDrive" => Ok(ONEDRIVE_TOKEN_ENDPOINT.to_string()),
        "GoogleDrive" => Ok(GOOGLEDRIVE_TOKEN_ENDPOINT.to_string()),
        _ => Err(anyhow::anyhow!("unsupported provider \"{provider}\"")),
    }
}

/// Get the API OAuth2 scope for the given cloud provider. Currently
/// supported options are "OneDrive" and "GoogleDrive".
pub fn scope(provider: &str) -> Result<String> {
    match provider {
        "OneDrive" => Ok(ONEDRIVE_SCOPE.to_string()),
        "GoogleDrive" => Ok(GOOGLEDRIVE_SCOPE.to_string()),
        _ => Err(anyhow::anyhow!("unsupported provider \"{provider}\"")),
    }
}

/// The OneDrive API authentication endpoint for obtaining
/// authorization grants.
pub const ONEDRIVE_AUTH_ENDPOINT: &str =
    "https://login.microsoftonline.com/common/oauth2/v2.0/authorize";

/// The OneDrive API token endpoint for obtaining and refreshing
/// access tokens.
pub const ONEDRIVE_TOKEN_ENDPOINT: &str =
    "https://login.microsoftonline.com/common/oauth2/v2.0/token";

/// The Microsoft Graph OAuth2 scope required for accessing OneDrive
/// resources.
pub const ONEDRIVE_SCOPE: &str = "files.readwrite.all offline_access";

/// The Google Drive API authentication endpoint for obtaining
/// authorization grants.
pub const GOOGLEDRIVE_AUTH_ENDPOINT: &str =
    "https://accounts.google.com/o/oauth2/v2/auth";

/// The Google Drive API token endpoint for obtaining and refreshing
/// access tokens.
pub const GOOGLEDRIVE_TOKEN_ENDPOINT: &str =
    "https://oauth2.googleapis.com/token";

/// The Google OAuth2 scope required for accessing OneDrive resources.
pub const GOOGLEDRIVE_SCOPE: &str =
    "https://www.googleapis.com/auth/drive";
