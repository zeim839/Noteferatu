use crate::oauth2::{Config, Token};
use super::utils::handle_read_lock_err;

use anyhow::{Context, Result, anyhow};
use reqwest::{Response, RequestBuilder, Error, header};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::time::sleep;

pub(crate) struct Client {
    token: Arc<RwLock<Token>>,
    config: Arc<Config>,
}

impl Client {

    /// Initializes a new client.
    pub fn new(token: &Token, config: &Config) -> Self {
        Self {
            token: Arc::new(RwLock::new(token.clone())),
            config: Arc::new(config.clone()),
        }
    }

    /// Check if the access token has expired and refreshes if needed.
    pub async fn refresh_if_expired(&self) -> Result<()> {

        // Use read lock to opportunistically check if expired.
        {
            let token = self.token.read().map_err(handle_read_lock_err)?;
            if !token.is_expired() {
                return Ok(());
            }
        }

        // Refresh access token.
        let mut token = self.token.write()
            .map_err(handle_read_lock_err)?;

        // Double-check pattern.
        token.refresh_if_expired(&self.config).await
            .context("Failed to refresh OAuth2 token")?;

        Ok(())
    }

    /// Executes a request, retrying up to 3 times for certain errors
    /// using exponential backoff.
    pub async fn execute_with_retry(&self, req: RequestBuilder) -> Result<Response> {
        const MAX_RETRIES: u32 = 3;
        let mut last_error = None;
        for attempt in 0..MAX_RETRIES {
            let request = req.try_clone()
                .ok_or_else(|| anyhow!("request body is not clonable"))?;

            match request.send().await {
                Ok(res) => {
                    if should_retry_status(res.status()) {
                        if attempt < MAX_RETRIES - 1 {
                            let delay = Duration::from_millis(250 * (2_u64.pow(attempt)));
                            sleep(delay).await;
                            continue;
                        }
                    }
                    return Ok(res);
                },
                Err(err) => {
                    if should_retry_error(&err) {
                        last_error = Some(err);
                        if attempt < MAX_RETRIES - 1 {
                            let delay = Duration::from_millis(250 * (2_u64.pow(attempt)));
                            sleep(delay).await;
                            continue;
                        }
                    } else {return Err(anyhow!("{err}"));}
                }
            }
        }

        return Err(anyhow!("{}", last_error.unwrap()));
    }

    /// Acquires a Bearer authentication string, refreshing the
    /// underlying token if needed.
    pub async fn bearer(&self) -> Result<String> {
        self.refresh_if_expired().await?;
        let token = self.token.read().map_err(handle_read_lock_err)?;
        return Ok(format!("Bearer {}", token.access_token));
    }
}

/// Constructs a reqwest client and sets its timeout and OAuth2
/// authorization header.
fn build_client(token: &Token) -> reqwest::Client {
    let mut headers = header::HeaderMap::new();
    let bearer_str = format!("Bearer {}", &token.access_token);
    let bearer = header::HeaderValue::from_str(&bearer_str).unwrap();
    headers.insert(header::AUTHORIZATION, bearer);
    return reqwest::Client::builder()
        .default_headers(headers)
        .user_agent("helsync/1.0")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();
}

/// Determines if a status code should trigger a retry
fn should_retry_status(status: reqwest::StatusCode) -> bool {
    matches!(
        status,
        reqwest::StatusCode::UNAUTHORIZED |           // 401
        reqwest::StatusCode::NOT_FOUND |              // 404
        reqwest::StatusCode::REQUEST_TIMEOUT |        // 408
        reqwest::StatusCode::TOO_MANY_REQUESTS |      // 429
        reqwest::StatusCode::INTERNAL_SERVER_ERROR |  // 500
        reqwest::StatusCode::BAD_GATEWAY |            // 502
        reqwest::StatusCode::SERVICE_UNAVAILABLE |    // 503
        reqwest::StatusCode::GATEWAY_TIMEOUT          // 504
    )
}

/// Determines if an error should trigger a retry
fn should_retry_error(error: &Error) -> bool {
    // Retry on network errors, timeouts, etc.
    error.is_timeout() ||
    error.is_connect() ||
    error.is_request()
}
