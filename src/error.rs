use std::fmt;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

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

    ResponseText,
    ResponseJson,

    CsrfTokenMismatch,

    FormData,
    OAuthError,

    ClientSetup,
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
        matches!(self.inner.kind, |Kind::CsrfTokenMismatch| Kind::OAuthError)
    }

    pub fn is_validation_error(&self) -> bool {
        matches!(
            self.inner.kind,
            Kind::ResponseText | Kind::ResponseJson | Kind::FormData
        )
    }

    pub fn is_retryable(&self) -> bool {
        matches!(self.inner.kind, Kind::Request)
    }

    pub fn is_response_parsing_error(&self) -> bool {
        matches!(self.inner.kind, Kind::ResponseText | Kind::ResponseJson)
    }

    pub fn is_client_setup_error(&self) -> bool {
        matches!(self.inner.kind, Kind::ClientSetup)
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

impl From<asknothingx2_util::api::Error> for Error {
    fn from(value: asknothingx2_util::api::Error) -> Self {
        Self::with_source(Kind::Request, value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::with_source(Kind::Request, value)
    }
}

impl Kind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Kind::Request => "network request failed",
            Kind::ResponseText => "failed to read response as text",
            Kind::ResponseJson => "failed to parse response as JSON",
            Kind::CsrfTokenMismatch => "CSRF token mismatch",
            Kind::FormData => "failed to serialize form data",
            Kind::OAuthError => "OAuth error response",
            Kind::ClientSetup => "HTTP client setup failed",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenError {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OAuth error {}: {}", self.status, self.message)?;
        if let Some(ref error) = self.error {
            write!(f, " ({error})")?;
        }
        Ok(())
    }
}

impl std::error::Error for TokenError {}

pub mod network {
    use super::{BoxError, Error, Kind};

    pub fn request<E: Into<BoxError>>(e: E) -> Error {
        Error::with_source(Kind::Request, e)
    }
}

pub mod oauth {

    use super::{Error, Kind};

    pub fn csrf_token_mismatch() -> Error {
        Error::with_message(
            Kind::CsrfTokenMismatch,
            "CSRF token validation failed - possible security issue",
        )
    }

    pub fn server_error(message: impl Into<String>) -> Error {
        Error::with_message(Kind::OAuthError, message)
    }
}

pub mod validation {
    use super::{BoxError, Error, Kind};

    pub fn form_data<E: Into<BoxError>>(source: E) -> Error {
        Error::with_source(Kind::FormData, source)
    }
}

pub mod response {
    use super::{BoxError, Error, Kind};

    pub fn text<E: Into<BoxError>>(e: E) -> Error {
        Error::with_source(Kind::ResponseText, e)
    }

    pub fn json<E: Into<BoxError>>(e: E) -> Error {
        Error::with_source(Kind::ResponseJson, e)
    }
}

pub mod client_setup {
    use super::{BoxError, Error, Kind};

    pub fn already_initialized() -> Error {
        Error::with_message(
            Kind::ClientSetup,
            "HTTP client has already been initialized and cannot be reconfigured",
        )
    }

    pub fn build_failed<E: Into<BoxError>>(source: E) -> Error {
        Error::with_message_and_source(
            Kind::ClientSetup,
            "failed to build HTTP client from preset configuration",
            source,
        )
    }

    pub fn from_preset_error<M: Into<String>>(
        message: M,
        source: asknothingx2_util::api::Error,
    ) -> Error {
        Error::with_message_and_source(Kind::ClientSetup, message, source)
    }
}
