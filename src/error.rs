use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

use serde::Deserialize;

pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

pub struct Error {
    inner: Box<Inner>,
}

struct Inner {
    kind: Kind,
    operation: Operation,
    source: Option<BoxError>,
}

enum Kind {
    Build,
    Request,
    Body,
    Api(Api),
    Parse,
    Csrf,
    DeviceCodeExpired,
}

struct Api {
    status: u16,
    message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    Build,
    Request,
    Api,
    Parse,
    Csrf,
    DeviceCodeExpired,
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::Request => "request",
            Self::Api => "api",
            Self::Parse => "parse",
            Self::Csrf => "csrf",
            Self::DeviceCodeExpired => "device_code_expired",
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl Error {
    pub fn is_timeout(&self) -> bool {
        self.inner
            .source
            .as_ref()
            .and_then(|e| e.downcast_ref::<reqwest::Error>())
            .is_some_and(reqwest::Error::is_timeout)
    }

    pub fn kind(&self) -> ErrorKind {
        match self.inner.kind {
            Kind::Build => ErrorKind::Build,
            Kind::Request | Kind::Body => ErrorKind::Request,
            Kind::Api(_) => ErrorKind::Api,
            Kind::Parse => ErrorKind::Parse,
            Kind::Csrf => ErrorKind::Csrf,
            Kind::DeviceCodeExpired => ErrorKind::DeviceCodeExpired,
        }
    }

    pub fn status(&self) -> Option<u16> {
        self.api().map(|api| api.status)
    }

    pub fn message(&self) -> Option<&str> {
        self.api().and_then(|api| api.message.as_deref())
    }

    fn api(&self) -> Option<&Api> {
        match &self.inner.kind {
            Kind::Api(api) => Some(api),
            _ => None,
        }
    }

    fn new(kind: Kind, operation: Operation, source: Option<BoxError>) -> Self {
        Self {
            inner: Box::new(Inner {
                kind,
                operation,
                source,
            }),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut builder = f.debug_struct("twitch_oauth_token::Error");

        builder.field("kind", &self.kind());
        builder.field("operation", &self.inner.operation.as_str());

        if let Kind::Api(api) = &self.inner.kind {
            builder.field("status", &api.status);
            if let Some(message) = &api.message {
                builder.field("message", message);
            }
        }

        if let Some(source) = &self.inner.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.inner.kind {
            Kind::Build => f.write_str("failed to build request")?,
            Kind::Request => f.write_str("failed to send request")?,
            Kind::Body => f.write_str("failed to read response")?,
            Kind::Api(api) => {
                write!(f, "unexpected HTTP status {}", api.status)?;
                match api.message.as_deref() {
                    Some(message) if !message.is_empty() => write!(f, " {message:?}")?,
                    _ => {}
                }
            }
            Kind::Parse => f.write_str("failed to deserialize response")?,
            Kind::Csrf => f.write_str("failed to verify CSRF state")?,
            Kind::DeviceCodeExpired => f.write_str("device code expired")?,
        }

        write!(f, " for {}", self.inner.operation)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

#[derive(Copy, Clone)]
pub(crate) enum Operation {
    AppAccessToken,
    ExchangeCode,
    RefreshAccessToken,
    RevokeAccessToken,
    ValidateAccessToken,
    DeviceRequest,
    DevicePoll,
    #[cfg(feature = "test")]
    MockUserAccessToken,
    #[cfg(feature = "test")]
    MockApiUnits,
}

impl Operation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AppAccessToken => "app_access_token",
            Self::ExchangeCode => "exchange_code",
            Self::RefreshAccessToken => "refresh_access_token",
            Self::RevokeAccessToken => "revoke_access_token",
            Self::ValidateAccessToken => "validate_access_token",
            Self::DeviceRequest => "device_auth_request",
            Self::DevicePoll => "device_auth_poll",
            #[cfg(feature = "test")]
            Self::MockUserAccessToken => "mock_user_access_token",
            #[cfg(feature = "test")]
            Self::MockApiUnits => "mock_api_units",
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

pub(crate) fn build(operation: Operation, source: impl Into<BoxError>) -> Error {
    Error::new(Kind::Build, operation, Some(source.into()))
}

pub(crate) fn request(operation: Operation, source: reqwest::Error) -> Error {
    let kind = if source.is_builder() {
        Kind::Build
    } else {
        Kind::Request
    };

    Error::new(kind, operation, Some(source.without_url().into()))
}

pub(crate) fn body(operation: Operation, source: reqwest::Error) -> Error {
    Error::new(Kind::Body, operation, Some(source.without_url().into()))
}

pub(crate) fn api(operation: Operation, status: u16, body: &[u8]) -> Error {
    #[derive(Deserialize)]
    struct ApiBody {
        message: Option<String>,
    }

    let message = serde_json::from_slice::<ApiBody>(body)
        .ok()
        .and_then(|body| body.message);

    Error::new(Kind::Api(Api { status, message }), operation, None)
}

pub(crate) fn parse(operation: Operation, source: impl Into<BoxError>) -> Error {
    Error::new(Kind::Parse, operation, Some(source.into()))
}

pub(crate) fn csrf(operation: Operation, source: impl Into<BoxError>) -> Error {
    Error::new(Kind::Csrf, operation, Some(source.into()))
}

pub(crate) fn device_code_expired(operation: Operation) -> Error {
    Error::new(Kind::DeviceCodeExpired, operation, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_sync_static() {
        fn assert<T: Send + Sync + 'static>() {}
        assert::<Error>();
    }
}
