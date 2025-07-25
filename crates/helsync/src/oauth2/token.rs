use crate::errors::Result;
use super::Config;

use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::header::CONTENT_TYPE;
use serde::{Serialize, Deserialize};

/// OAuth2 access and refresh tokens.
///
/// # Examples
/// Creates an [Token] using struct instantiation syntax (see
/// also: [Token::from_refresh_token]) and refreshes it if
/// expired.
/// ```no_run
/// use helsync::oauth2;
///
/// #[tokio::main]
/// async fn main() {
///     let app_config = oauth2::Config::onedrive(
///         "client-id", "http://localhost:6969"
///     );
///
///     let mut token = oauth2::Token {
///         access_token: "my-expired-access-token".to_string(),
///         refresh_token: "my-valid-refresh-token".to_string(),
///         created_at: 0,
///         expires_in: 0,
///     };
///
///     // Refresh if expired.
///     if token.is_expired() {
///         token.refresh(&app_config).await.unwrap();
///     }
///
///     // Or, equivalently:
///     token.refresh_if_expired(&app_config).await.unwrap();
/// }
/// ```
#[derive(Clone)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub created_at: i64,
    pub expires_in: i64,
}

impl Token {

    /// Derive a [Token] from a refresh token. Useful when only
    /// the refresh token has been preserved and you need to
    /// re-authenticate.
    pub async fn from_refresh_token(
        refresh_token: &str,
        app: &Config
    ) -> Result<Self> {
        let mut token = Token {
            access_token: String::new(),
            refresh_token: refresh_token.to_string(),
            created_at: 0,
            expires_in: 0,
        };
        token.refresh(app).await?;
        Ok(token)
    }

    /// Refreshes the [Token].
    pub async fn refresh(&mut self, config: &Config) -> Result<()> {
        let client_secret = config.client_secret.clone()
            .unwrap_or(String::new());

        let mut params = vec![
            ("grant_type", "refresh_token"),
            ("client_id", &config.client_id),
            ("redirect_uri", &config.redirect_uri),
            ("refresh_token", &self.refresh_token),
        ];

        if client_secret != "" {
            params.push(("client_secret", &client_secret));
        }

        let res = reqwest::Client::new()
            .post(&config.token_endpoint)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&params)
            .send().await?
            .error_for_status()?;

        let token: TokenResponse = res.json().await?;
        let created_at: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as i64;

        self.created_at = created_at;
        self.expires_in = token.expires_in.unwrap_or(1200);
        self.access_token = token.access_token;
        self.refresh_token = token.refresh_token
            .unwrap_or(self.refresh_token.clone());

        Ok(())
    }

    /// Checks whether the [Token] has expired.
    pub fn is_expired(&self) -> bool {
        let now: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        now >= self.created_at + self.expires_in
    }

    /// Refreshes the [Token], but only if expired.
    pub async fn refresh_if_expired(&mut self, config: &Config) -> Result<()> {
        if self.is_expired() {self.refresh(config).await} else {Ok(())}
    }
}

/// Used for parsing JSON responses into structured OAuth2 tokens. A
/// TokenResponse uses Option to account for the possibility of
/// missing fields. [Token] is what's actually used for
/// configuring a client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub expires_in: Option<i64>,
    pub scope: Option<String>,
    pub refresh_token: Option<String>,
    pub created_at: Option<i64>,
}

impl Into<Token> for TokenResponse {
    fn into(self) -> Token {
        Token {
            access_token: self.access_token,
            refresh_token: self.refresh_token.unwrap_or_default(),
            created_at: self.created_at.unwrap_or_default(),
            expires_in: self.expires_in.unwrap_or_default(),
        }
    }
}
