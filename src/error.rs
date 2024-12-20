use crate::types::ErrorResponse;

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
    #[error("Operation timed out: {0}")]
    TimeoutError(String),
    #[error("Server is shutting down gracefully")]
    GraceFulShutdown,
    #[error("OAuth response missing required authorization code")]
    MissingAuthorizationCode,
}
