use std::fmt;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct Error {
    inner: Box<Inner>,
}

#[derive(Debug)]
struct Inner {
    kind: Kind,
    message: Option<String>,
    source: Option<BoxError>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Request,
    Timeout,
    Connect,

    InvalidRedirectUrl,
    MissingRedirectUrl,
    CsrfTokenMismatch,
    MissingCsrfToken,
    MissingAuthCode,
    InvalidScope,
    InvalidCredentials,
    TokenExpired,
    InvalidToken,

    ServerBind,
    ServerTimeout,
    ServerShutdown,
    ServerIo,

    UrlParse,
    InvalidHost,
    MissingHost,
    MissingQueryParam,
    Json,
}

impl Error {
    pub fn new(kind: Kind) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: None,
                source: None,
            }),
        }
    }

    pub fn with_message(kind: Kind, message: impl Into<String>) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: Some(message.into()),
                source: None,
            }),
        }
    }

    pub fn with_source(kind: Kind, source: impl Into<BoxError>) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: None,
                source: Some(source.into()),
            }),
        }
    }

    pub fn with_message_and_source(
        kind: Kind,
        message: impl Into<String>,
        source: impl Into<BoxError>,
    ) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: Some(message.into()),
                source: Some(source.into()),
            }),
        }
    }

    pub fn message(&self) -> Option<&str> {
        self.inner.message.as_deref()
    }

    pub fn is_network_error(&self) -> bool {
        matches!(
            self.inner.kind,
            Kind::Request | Kind::Timeout | Kind::Connect | Kind::ServerBind | Kind::ServerTimeout
        )
    }

    pub fn is_oauth_error(&self) -> bool {
        matches!(
            self.inner.kind,
            Kind::InvalidRedirectUrl
                | Kind::MissingRedirectUrl
                | Kind::CsrfTokenMismatch
                | Kind::MissingCsrfToken
                | Kind::MissingAuthCode
                | Kind::InvalidScope
                | Kind::InvalidCredentials
                | Kind::TokenExpired
                | Kind::InvalidToken
        )
    }

    pub fn is_validation_error(&self) -> bool {
        matches!(
            self.inner.kind,
            Kind::UrlParse | Kind::InvalidHost | Kind::MissingQueryParam | Kind::Json
        )
    }

    pub fn is_server_error(&self) -> bool {
        matches!(
            self.inner.kind,
            Kind::ServerBind | Kind::ServerTimeout | Kind::ServerShutdown | Kind::ServerIo
        )
    }

    pub fn is_retryable(&self) -> bool {
        matches!(
            self.inner.kind,
            Kind::Request | Kind::Timeout | Kind::Connect | Kind::ServerTimeout
        )
    }

    pub fn is_cancelled(&self) -> bool {
        matches!(self.inner.kind, Kind::ServerShutdown)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("twitch_oauth_token::Error");

        builder.field("kind", &self.inner.kind);

        if let Some(ref message) = self.inner.message {
            builder.field("message", message);
        }

        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref message) = self.inner.message {
            write!(f, "{message}")
        } else {
            write!(f, "{}", self.inner.kind)
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::with_source(Kind::UrlParse, value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::with_source(Kind::Json, value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        if value.kind() == std::io::ErrorKind::TimedOut {
            Self::new(Kind::Timeout)
        } else {
            Self::with_source(Kind::ServerIo, value)
        }
    }
}

#[cfg(feature = "oauth")]
impl From<asknothingx2_util::api::Error> for Error {
    fn from(value: asknothingx2_util::api::Error) -> Self {
        Self::with_source(Kind::Request, value)
    }
}

impl Kind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Kind::Request => "network request failed",
            Kind::Timeout => "operation timed out",
            Kind::Connect => "failed to connect",
            Kind::InvalidRedirectUrl => "invalid redirect URL",
            Kind::MissingRedirectUrl => "missing redirect URL",
            Kind::CsrfTokenMismatch => "CSRF token mismatch",
            Kind::MissingCsrfToken => "missing CSRF token",
            Kind::MissingAuthCode => "missing authorization code",
            Kind::InvalidScope => "invalid OAuth scope",
            Kind::InvalidCredentials => "invalid client credentials",
            Kind::TokenExpired => "access token expired",
            Kind::InvalidToken => "invalid access token",
            Kind::ServerBind => "failed to bind server",
            Kind::ServerTimeout => "server timeout",
            Kind::ServerShutdown => "server shutdown",
            Kind::ServerIo => "server I/O error",
            Kind::UrlParse => "failed to parse URL",
            Kind::InvalidHost => "invalid host",
            Kind::MissingHost => "missing host",
            Kind::MissingQueryParam => "missing query parameter",
            Kind::Json => "JSON error",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

pub mod network {
    use std::time::Duration;

    use super::{BoxError, Error, Kind};

    pub fn request<E: Into<BoxError>>(e: E) -> Error {
        Error::with_source(Kind::Request, e)
    }

    pub fn timeout() -> Error {
        Error::new(Kind::Timeout)
    }

    pub fn timeout_with_duration(duration: Duration) -> Error {
        Error::with_message(
            Kind::Timeout,
            format!("operation timed out after {duration:?}"),
        )
    }

    pub fn connect<E: Into<BoxError>>(e: E) -> Error {
        Error::with_source(Kind::Connect, e)
    }
}

pub mod oauth {
    use super::{BoxError, Error, Kind};

    pub fn invalid_redirect_url<E: Into<BoxError>>(e: E) -> Error {
        Error::with_source(Kind::InvalidRedirectUrl, e)
    }

    pub fn missing_redirect_url() -> Error {
        Error::with_message(
            Kind::MissingRedirectUrl,
            "redirect URL is required for this operation",
        )
    }

    pub fn csrf_token_mismatch() -> Error {
        Error::with_message(
            Kind::CsrfTokenMismatch,
            "CSRF token validation failed - possible security issue",
        )
    }

    pub fn missing_csrf_token() -> Error {
        Error::with_message(
            Kind::MissingCsrfToken,
            "CSRF token is missing from OAuth response",
        )
    }

    pub fn missing_auth_code() -> Error {
        Error::with_message(
            Kind::MissingAuthCode,
            "authorization code is missing from OAuth response",
        )
    }

    pub fn invalid_scope(scope: &str) -> Error {
        Error::with_message(Kind::InvalidScope, format!("invalid OAuth scope: {scope}"))
    }

    pub fn invalid_credentials() -> Error {
        Error::with_message(Kind::InvalidCredentials, "client credentials are invalid")
    }

    pub fn token_expired() -> Error {
        Error::with_message(Kind::TokenExpired, "access token has expired")
    }

    pub fn invalid_token() -> Error {
        Error::with_message(Kind::InvalidToken, "access token is invalid")
    }
}

pub mod server {
    use super::{BoxError, Error, Kind};

    pub fn bind<E: Into<BoxError>>(address: &str, e: E) -> Error {
        Error::with_message_and_source(
            Kind::ServerBind,
            format!("failed to bind to address: {address}"),
            e,
        )
    }

    pub fn timeout() -> Error {
        Error::with_message(Kind::ServerTimeout, "server operation timed out")
    }

    pub fn shutdown() -> Error {
        Error::with_message(Kind::ServerShutdown, "server is shutting down")
    }
}

pub mod validation {
    use super::{BoxError, Error, Kind};

    pub fn invalid_host(host: &str) -> Error {
        Error::with_message(Kind::InvalidHost, format!("invalid host: {host}"))
    }

    pub fn missing_host() -> Error {
        Error::with_message(Kind::MissingHost, "missing host in OAuth redirect URL")
    }

    pub fn missing_query_param(param: &str) -> Error {
        Error::with_message(
            Kind::MissingQueryParam,
            format!("missing required query parameter: {param}"),
        )
    }

    pub fn json<E: Into<BoxError>>(e: E) -> Error {
        Error::with_source(Kind::Json, e)
    }
}
