use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub struct Error {
    inner: Box<Inner>,
}

#[derive(Debug)]
struct Inner {
    kind: Kind,
    message: Option<String>,
    source: Option<BoxError>,
    status_code: Option<u16>,
    raw: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Kind {
    Request,

    Decode,

    CsrfTokenMismatch,

    FormData,
    OAuthError,
    Device,

    ClientSetup,
}

impl Error {
    pub(crate) fn with_message(kind: Kind, message: impl Into<String>) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: Some(message.into()),
                source: None,
                status_code: None,
                raw: None,
            }),
        }
    }

    pub(crate) fn with_source(kind: Kind, source: impl Into<BoxError>) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: None,
                source: Some(source.into()),
                status_code: None,
                raw: None,
            }),
        }
    }

    pub(crate) fn with_decode(
        kind: Kind,
        source: impl Into<BoxError>,
        raw: impl Into<String>,
    ) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: None,
                source: Some(source.into()),
                status_code: None,
                raw: Some(raw.into()),
            }),
        }
    }

    pub(crate) fn with_http_error(kind: Kind, status: u16, body: impl Into<String>) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                message: Some(format!("HTTP {status} error")),
                source: None,
                status_code: Some(status),
                raw: Some(body.into()),
            }),
        }
    }

    pub fn message(&self) -> Option<&str> {
        self.inner.message.as_deref()
    }

    pub fn raw(&self) -> Option<&str> {
        self.inner.raw.as_deref()
    }

    pub fn status_code(&self) -> Option<u16> {
        self.inner.status_code
    }

    pub fn is_request_error(&self) -> bool {
        matches!(self.inner.kind, Kind::Request)
    }

    pub fn is_oauth_error(&self) -> bool {
        matches!(self.inner.kind, Kind::CsrfTokenMismatch | Kind::OAuthError)
    }

    pub fn is_client_setup_error(&self) -> bool {
        matches!(self.inner.kind, Kind::ClientSetup)
    }

    pub fn is_decode(&self) -> bool {
        matches!(self.inner.kind, Kind::Decode)
    }

    pub fn is_device_code_error(&self) -> bool {
        matches!(self.inner.kind, Kind::Device)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut builder = f.debug_struct("twitch_oauth_token::Error");

        builder.field("kind", &self.inner.kind);

        if let Some(ref message) = self.inner.message {
            builder.field("message", message);
        }

        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }

        if let Some(ref raw) = self.inner.raw {
            builder.field("raw", raw);
        }

        builder.finish()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.inner.message {
            Some(msg) => f.write_str(msg),
            None => f.write_str(self.inner.kind.as_str()),
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
            Kind::CsrfTokenMismatch => "CSRF token mismatch",
            Kind::FormData => "failed to serialize form data",
            Kind::OAuthError => "OAuth error response",
            Kind::Device => "device code flow error response",
            Kind::ClientSetup => "HTTP client setup failed",
            Kind::Decode => "failed to deserialize response",
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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
    use super::{Error, Kind};

    pub fn csrf_token_mismatch() -> Error {
        Error::with_message(
            Kind::CsrfTokenMismatch,
            "CSRF token validation failed - possible security issue",
        )
    }

    pub fn http_error(status: u16, body: impl Into<String>) -> Error {
        Error::with_http_error(Kind::OAuthError, status, body)
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

    pub fn decode<E: Into<BoxError>>(e: E, raw: impl Into<String>) -> Error {
        Error::with_decode(Kind::Decode, e, raw)
    }
}

pub mod client_setup {
    use super::{Error, Kind};

    pub fn already_initialized() -> Error {
        Error::with_message(
            Kind::ClientSetup,
            "HTTP client has already been initialized and cannot be reconfigured",
        )
    }
}

pub mod device_code {
    use super::{Error, Kind};

    pub fn flow_error(status: u16, message: impl Into<String>) -> Error {
        Error::with_http_error(Kind::Device, status, message.into())
    }

    pub fn timeout() -> Error {
        Error::with_message(Kind::Device, "device code expired")
    }
}
