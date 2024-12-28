use std::fmt;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error occurred: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Invalid OAuth redirect URI: {0}")]
    RedirectUrlError(String),
    #[error("Missing required URL query parameter: {0}")]
    UrlQueryFindError(String),
    #[error("HTTP request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Unsupported HTTP method: {0}")]
    MethodError(String),
    #[error("OAuth server returned error: {0}")]
    ResponseError(ErrorResponse),
    #[error("CSRF token validation failed")]
    CsrfTokenPartialEqError,
    #[error("OAuth response missing required CSRF token")]
    ResponseCsrfTokenError,
    #[error("Failed to bind network address: {0}")]
    GetSocketAddrError(String),
    #[error("Invalid OAuth redirect host: expected 'localhost', got '{0}'")]
    InvalidRedirectHost(String),
    #[error("Missing host in OAuth redirect URL")]
    MissingRedirectHost,
    #[error("Operation timed out: {0}")]
    TimeoutError(String),
    #[error("Server is shutting down gracefully")]
    GraceFulShutdown,
    #[error("OAuth response missing required authorization code")]
    MissingAuthorizationCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    pub error: Option<String>,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
