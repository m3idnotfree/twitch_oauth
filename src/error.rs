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

    InvalidRedirectUrl,
    MissingRedirectUrl,
    CsrfTokenMismatch,

    UrlParse,
    UrlEncode,
    Json,

    OAuthError,
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
        matches!(self.inner.kind, Kind::Request)
    }

    pub fn is_oauth_error(&self) -> bool {
        matches!(
            self.inner.kind,
            Kind::InvalidRedirectUrl
                | Kind::MissingRedirectUrl
                | Kind::CsrfTokenMismatch
                | Kind::OAuthError
        )
    }

    pub fn is_validation_error(&self) -> bool {
        matches!(self.inner.kind, Kind::UrlParse | Kind::Json)
    }

    pub fn is_retryable(&self) -> bool {
        matches!(self.inner.kind, Kind::Request)
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

#[cfg(feature = "oauth")]
impl From<asknothingx2_util::api::Error> for Error {
    fn from(value: asknothingx2_util::api::Error) -> Self {
        Self::with_source(Kind::Request, value)
    }
}

#[cfg(feature = "oauth")]
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::with_source(Kind::Request, value)
    }
}

impl Kind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Kind::Request => "network request failed",
            Kind::InvalidRedirectUrl => "invalid redirect URL",
            Kind::MissingRedirectUrl => "missing redirect URL",
            Kind::CsrfTokenMismatch => "CSRF token mismatch",
            Kind::UrlParse => "failed to parse URL",
            Kind::UrlEncode => "failed to encoding URL",
            Kind::Json => "failed to parse JSON",
            Kind::OAuthError => "OAuth error response",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

pub mod network {
    use super::{BoxError, Error, Kind};

    pub fn request<E: Into<BoxError>>(e: E) -> Error {
        Error::with_source(Kind::Request, e)
    }
}

pub mod oauth {
    use crate::TokenError;

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

    pub fn from_token_error(response: TokenError) -> Error {
        let message = if let Some(desc) = &response.error {
            format!("{}: {}, {}", response.status, response.message, desc)
        } else {
            format!("{}: {}", response.status, response.message)
        };
        Error::with_message(Kind::OAuthError, message)
    }
}

pub mod validation {
    use super::{BoxError, Error, Kind};

    pub fn url_encode<E: Into<BoxError>>(source: E) -> Error {
        Error::with_source(Kind::UrlEncode, source)
    }

    pub fn json<E: Into<BoxError>>(e: E) -> Error {
        Error::with_source(Kind::Json, e)
    }
}
