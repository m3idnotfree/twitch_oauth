#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] HttpError),
    #[error(transparent)]
    OAuth(#[from] OAuthError),
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error(transparent)]
    Server(#[from] ServerError),
}

#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    #[error(transparent)]
    RequestError(#[from] asknothingx2_util::api::ReqwestError),
    #[error(transparent)]
    JsonError(#[from] asknothingx2_util::api::JsonError),
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(String),
}

#[derive(thiserror::Error, Debug)]
pub enum OAuthError {
    #[error("Invalid OAuth redirect URI: {0}")]
    RedirectUri(String),
    #[error("Missing redirect URI")]
    MissingRedirectUri,
    #[error("Missing required URL query parameter: {0}")]
    MissingQueryParam(String),
    #[error("CSRF token validation failed")]
    CsrfTokenMismatch,
    #[error("OAuth response missing required CSRF token")]
    MissingCsrfToken,
    #[error("OAuth response missing required authorization code")]
    MissingAuthCode,
}

#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
    #[error("Failed to parse URL: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("Invalid OAuth redirect host: expected 'localhost', got '{0}'")]
    InvalidHost(String),
    #[error("Missing host in OAuth redirect URL")]
    MissingHost,
}

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("I/O error occurred: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to bind network address: {0}")]
    BindAddressError(String),
    #[error("Operation timed out: {0}")]
    Timeout(String),
    #[error("Server is shutting down gracefully")]
    Shutdown,
}
