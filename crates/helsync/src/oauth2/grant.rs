use super::pkce::PKCE;
use super::token::{Token, TokenResponse};
use super::Config;
use crate::errors::{Error, Result};

use std::collections::HashMap;
use std::process::Command as CMD;
use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::header::{CONTENT_TYPE, ACCEPT};

use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::TcpListener;
use tokio::sync::mpsc;

/// An OAuth2 authorization grant, which can be exchanged for an API
/// access token.
///
/// # Examples
/// The example starts a local HTTP server and opens the user's
/// browser agent to the OAuth2 authorization URL. Once the user
/// authenticates, an authorization code is granted and the user's
/// browser agent is redirected to redirect_uri.
///
/// Assuming the redirect_uri points to the local server, the user's
/// authorization grant is captured and is immediately exchanged for
/// an access token using [Grant::to_token].
///
/// ```no_run
/// use helsync::oauth2::{self, Config, Grant};
///
/// #[tokio::main]
/// async fn main() {
///
///     // Define the OAuth2 application registration.
///     let app_config = oauth2::Config::onedrive(
///         "client-id", "http://localhost:6969"
///     );
///
///     let token = Grant::from_server(6969, &app_config)
///         .await.unwrap()
///         .to_token()
///         .await.unwrap();
/// }
/// ```
pub struct Grant {
    pub code: String,
    pkce: PKCE,
    config: Config,
}

impl Grant {

    /// Initialize using a known authorization grant code.
    pub fn from_code(code: &str, config: &Config) -> Self {
        Self {
            code: code.to_string(),
            pkce: PKCE::new(),
            config: config.clone(),
        }
    }

    /// Initialize by requesting the user's
    /// [authorization](request_auth_grant) via browser agent and
    /// capture an authorization grant redirect using a local HTTP
    /// server that listen in the background.
    pub async fn from_server(port: u16, config: &Config) -> Result<Self> {
        let (tx, mut rx) = mpsc::channel(1024);
        tokio::spawn(async move {
            let params = listen_and_serve(port).await;
            if let Some(grant) = params.get("code") {
                let _ = tx.send(grant.to_string()).await;
            }
        });

        let pkce = PKCE::new();
        request_auth_grant(&pkce, &config)?;
        let grant = rx.recv().await
            .ok_or(Error::Other("failed to capture authorization grant".to_string()))?;

        Ok(Self {code: grant, pkce, config: config.clone()})
    }

    /// Exchange the authorization grant for an OAuth2 [Token].
    pub async fn to_token(&self) -> Result<Token> {
        let client_secret = self.config.client_secret.clone()
            .unwrap_or(String::new());

        let mut params = vec![
            ("grant_type", "authorization_code"),
            ("client_id", &self.config.client_id),
            ("code", &self.code),
            ("redirect_uri", &self.config.redirect_uri),
            ("code_verifier", &self.pkce.verifier),
        ];

        if client_secret != "" {
            params.push(("client_secret", &client_secret));
        }

        let res = reqwest::Client::new()
            .post(&self.config.token_endpoint)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(ACCEPT, "application/json")
            .form(&params)
            .send().await?
            .error_for_status()?;

        let mut token: TokenResponse = res.json().await?;
        let created_at: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as i64;

        token.created_at = Some(created_at);
        token.expires_in = Some(token.expires_in.unwrap_or(1200));
        Ok(token.into())
    }
}

/// Request user authorization by opening the user browser agent to
/// the authorization URL constructed from the given AppConfig.
///
/// # Examples
/// Configure an API app registration and open the user browser agent
/// to the OAuth2 authorization URL:
///
/// ```no_run
/// use helsync::oauth2::{Config, PKCE, request_auth_grant};
///
/// // Define the OAuth2 application registration.
/// let app_config = Config::onedrive(
///     "client-id", "http://localhost:6969"
/// );
///
/// // Create PKCE challenge.
/// let pkce = PKCE::new();
///
/// request_auth_grant(&pkce, &app_config).unwrap();
/// ```
pub fn request_auth_grant(pkce: &PKCE, config: &Config) -> Result<()> {
    let auth_url = format!("{}?{}", config.auth_endpoint,
        form_urlencoded::Serializer::new(String::new())
            .append_pair("response_type", "code")
            .append_pair("client_id", &config.client_id)
            .append_pair("redirect_uri", &config.redirect_uri)
            .append_pair("code_challenge", &pkce.challenge)
            .append_pair("code_challenge_method", &pkce.method)
            .append_pair("scope", &config.scope)
            .finish()
    );
    open_browser(&auth_url);
    Ok(())
}

/// Listens for incoming authorization redirects on the given
/// port, returning a hashmap of the request's query parameters.
async fn listen_and_serve(port: u16) -> HashMap<String, String> {
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await
        .expect("could not start authorization grant server");

    let (stream, _) = listener.accept().await
        .expect("could not accept incoming authorization redirect");

    let mut stream = BufStream::new(stream);
    let req = parse_request(&mut stream).await
        .expect("failed to parse request");

    send_html_response(&mut stream, HTML_RESPONSE).await
        .expect("failed to respond to request");

    return req;
}

/// Open the user's browser to the given URL.
fn open_browser(url: &str) {
    #[cfg(target_os = "windows")]
    CMD::new("cmd")
        .args(&["/C", "start", url])
        .spawn()
        .expect("failed to open browser");

    #[cfg(target_os = "linux")]
    CMD::new("xdg-open")
        .arg(url)
        .spawn()
        .expect("failed to open browser");

    #[cfg(target_os = "macos")]
    CMD::new("open")
        .arg(url)
        .spawn()
        .expect("failed to open browser");

    println!("your browser has been opened to visit:\n\t{url}");
}

/// Parses an incoming (presumed) authorization request and returns a
/// map of its URL query parameters. The authorization code may then
/// be extracted from the "code" key.
async fn parse_request(
    mut stream: impl AsyncBufRead + Unpin
) -> Result<HashMap<String, String>> {
    let mut line_buffer = String::new();
    stream.read_line(&mut line_buffer).await?;

    let mut parts = line_buffer.split_whitespace();
    let method = parts.next()
        .ok_or(Error::Other("oauth2: missing method".to_string()))?;

    if method != "GET" {
        return Err(Error::Other(format!("oauth2: unsupported method: {}", method)));
    }

    let path: String = parts.next()
        .ok_or(Error::Other("oauth2: missing path".to_string()))
        .map(Into::into)?;

    Ok(extract_query_params(&path))
}

/// Sends a polite response to the authorization redirect that lets
/// the user know that it's safe to close the page.
async fn send_html_response(
    stream: &mut BufStream<tokio::net::TcpStream>,
    html_content: &str
) -> Result<()> {
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        html_content.len(),
        html_content
    );

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

/// Extracts the query parameters from a URL string.
fn extract_query_params(uri: &str) -> HashMap<String, String> {
    let query_string = uri.split('?').nth(1).unwrap_or("");
    let mut params = HashMap::new();
    for param in query_string.split('&') {
        let mut pair = param.splitn(2, '=');
        let key = pair.next().unwrap_or_default();
        let value = pair.next().unwrap_or_default();
        params.insert(key.to_string(), value.to_string());
    }
    params
}

const HTML_RESPONSE: &str = "
<html>
  <body>
    <p>Authentication successful.</p>
    <p>You may safely close this page.</p>
  </body>
</html>
";
