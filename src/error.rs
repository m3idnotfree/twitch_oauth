use crate::types::ErrorResponse;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Io error:")]
    IoError(#[from] std::io::Error),
    #[error("invalid url: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("invalid redirect_uri: {0}")]
    RedirectUrlError(anyhow::Error),
    #[error("can't find query: {0}")]
    UrlQueryFindError(String),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("is Method not implement: {0}")]
    MethodError(String),
    #[error("error response: {0}")]
    ResponseError(ErrorResponse),
    #[error("csrf token partialeq error")]
    CsrfTokenPartialEqError,
    #[error("can't get SocketAddr: {0}")]
    GetSocketAddrError(String),
    #[error("timeout error")]
    TimeoutError(String),
    #[error("graceful shutdown")]
    GraceFulShutdown(String),
}
