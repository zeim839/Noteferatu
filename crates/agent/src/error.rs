use std::fmt::{Display, Formatter, Result};
use serde::{Serialize, Deserialize};

/// An LLM client error.
#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    #[serde(rename = "type")]
    pub kind: String,
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
     }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        if error.is_builder() {
            Error {
                kind: "CLIENT_BUILDER_ERR".to_string(),
                message: "invalid request builder".to_string(),
            }
        } else if error.is_redirect() {
            Error {
                kind: "CLIENT_REDIRECT_POLICY_ERR".to_string(),
                message: "request violated client redirect policy".to_string(),
            }
        } else if error.is_status() {
            Error {
                kind: format!("HTTP_STATUS_{}_ERR", error.status().unwrap()),
                message: error.to_string(),
            }
        } else if error.is_timeout() {
            Error {
                kind: "CLIENT_TIMEOUT_ERR".to_string(),
                message: "client has timed out while making a request".to_string(),
            }
        } else if error.is_request() {
            Error {
                kind: "CLIENT_REQUEST_ERR".to_string(),
                message: error.to_string(),
            }
        } else if error.is_connect() {
            Error {
                kind: "CLIENT_CONNECT_ERR".to_string(),
                message: "client could not connect to the internet".to_string(),
            }
        } else if error.is_body() || error.is_decode() {
            Error {
                kind: "CLIENT_BODY_ERR".to_string(),
                message: "client could not decode response body".to_string(),
            }
        } else {
            Error {
                kind: "CLIENT_UNKNOWN_ERR".to_string(),
                message: "unknown error".to_string(),
            }
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error {
            kind: "JSON_ERR".to_string(),
            message: error.to_string(),
        }
    }
}

impl std::error::Error for Error {}
