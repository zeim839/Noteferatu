/// Get the OAuth2 authorization endpoint for the given cloud
/// provider. Currently supported options are "OneDrive" and
/// "GoogleDrive".
///
/// # Panics
/// Panics if `provider` is neither `OneDrive` nor `GoogleDrive`.
pub fn auth_endpoint(provider: &str) -> String {
    match provider.to_lowercase().as_str() {
        "onedrive" => ONEDRIVE_AUTH_ENDPOINT.to_string(),
        "googledrive" => GOOGLEDRIVE_AUTH_ENDPOINT.to_string(),
        _ => panic!("unsupported provider \"{provider}\""),
    }
}

/// Get the OAuth2 token endpoint for the given cloud
/// provider. Currently supported options are "OneDrive" and
/// "GoogleDrive".
///
/// # Panics
/// Panics if `provider` is neither `OneDrive` nor `GoogleDrive`.
pub fn token_endpoint(provider: &str) -> String {
    match provider.to_lowercase().as_str() {
        "onedrive" => ONEDRIVE_TOKEN_ENDPOINT.to_string(),
        "googledrive" => GOOGLEDRIVE_TOKEN_ENDPOINT.to_string(),
        _ => panic!("unsupported provider \"{provider}\""),
    }
}

/// Get the API OAuth2 scope for the given cloud provider. Currently
/// supported options are "OneDrive" and "GoogleDrive".
///
/// # Panics
/// Panics if `provider` is neither `OneDrive` nor `GoogleDrive`.
pub fn scope(provider: &str) -> String {
    match provider.to_lowercase().as_str() {
        "onedrive" => ONEDRIVE_SCOPE.to_string(),
        "googledrive" => GOOGLEDRIVE_SCOPE.to_string(),
        _ => panic!("unsupported provider \"{provider}\""),
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
