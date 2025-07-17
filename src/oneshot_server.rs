use std::{fmt, io, net::SocketAddr, time::Duration};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{TcpListener, TcpStream},
    time::timeout,
};

use crate::{types::OAuthCallbackQuery, AuthorizationCode};

/// only support localhost
pub async fn oneshot_server(
    addr: &str,
    duration: Duration,
) -> Result<OAuthCallbackQuery, ServerError> {
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|source| ServerError::BindFailed {
            addr: addr.to_string(),
            source,
        })?;

    let mut signal = std::pin::pin!(shutdown_signal());

    tokio::select! {
        rev = timeout(duration, listener.accept()) => {
            handle_connection_result(rev).await
        },
        _ = &mut signal => {
            Err(ServerError::Shutdown)
        },
    }
}

async fn handle_connection_result(
    rev: Result<Result<(TcpStream, SocketAddr), io::Error>, tokio::time::error::Elapsed>,
) -> Result<OAuthCallbackQuery, ServerError> {
    match rev {
        Ok(res) => {
            let (stream, _addr) = res.map_err(ServerError::RequestReadFailed)?;
            let (code, state, scope) = code_state_parse(stream).await?;
            Ok(OAuthCallbackQuery {
                code: AuthorizationCode::new(code),
                state,
                scope,
            })
        }
        Err(_) => Err(ServerError::Timeout),
    }
}

/// Wait for the CTRL+C signal
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn code_state_parse(mut stream: TcpStream) -> Result<(String, String, String), ServerError> {
    let mut reader = BufReader::new(&mut stream);

    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .await
        .map_err(ServerError::RequestReadFailed)?;

    let trimmed = request_line.trim();
    if trimmed.is_empty() {
        return Err(ServerError::InvalidRequest("empty request".to_string()));
    }

    // [METHOD] [PATH][?QUERY_PARAMS] [HTTP_VERSION]\r\n
    // GET /?code=...&scope=...&state=.... HTTP/1.1\r\n
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(ServerError::InvalidRequest(format!(
            "invalid HTTP request format: expected 'METHOD PATH VERSION', got: '{trimmed}'"
        )));
    }

    let method = parts[0];
    let path = parts[1];
    let version = parts[2];

    if method != "GET" {
        return Err(ServerError::InvalidRequest(format!(
            "OAuth callback must be GET, got {method}",
        )));
    }

    if !version.starts_with("HTTP/") {
        return Err(ServerError::InvalidRequest(format!(
            "invalid HTTP version: {version}",
        )));
    }

    let query_string = path.split_once('?').map(|(_, query)| query).unwrap_or("");

    if query_string.contains("error=") {
        return Err(ServerError::OAuthError(query_string.to_string()));
    }

    let mut code = None;
    let mut state = None;
    let mut scope = None;

    for pair in query_string.split('&') {
        if pair.is_empty() {
            continue;
        }

        if let Some((key, value)) = pair.split_once('=') {
            match key {
                "code" => {
                    if code.is_none() {
                        code = Some(value.to_string());
                    }
                }
                "state" => {
                    if state.is_none() {
                        state = Some(value.to_string());
                    }
                }
                "scope" => {
                    if scope.is_none() {
                        scope = Some(value.to_string());
                    }
                }
                _ => {}
            }
        }
    }

    let code = code.ok_or_else(|| ServerError::MissingQueryParam {
        param: "code".to_string(),
    })?;

    let state = state.ok_or_else(|| ServerError::MissingQueryParam {
        param: "state".to_string(),
    })?;

    let scope = scope.ok_or_else(|| ServerError::MissingQueryParam {
        param: "scope".to_string(),
    })?;

    if code.is_empty() {
        return Err(ServerError::InvalidRequest(
            "OAuth 'code' parameter cannot be empty".to_string(),
        ));
    }

    if state.is_empty() {
        return Err(ServerError::InvalidRequest(
            "OAuth 'state' parameter cannot be empty".to_string(),
        ));
    }

    Ok((code, state, scope))
}

#[derive(Debug)]
pub enum ServerError {
    BindFailed {
        addr: String,
        source: std::io::Error,
    },
    Shutdown,
    Timeout,
    RequestReadFailed(std::io::Error),
    InvalidRequest(String),
    OAuthError(String),
    UrlParseFailed(url::ParseError),

    MissingQueryParam {
        param: String,
    },
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BindFailed { addr, source } => {
                write!(f, "failed to bind to address '{addr}': {source}")
            }
            Self::Shutdown => write!(f, "server received shutdown signal"),
            Self::Timeout => write!(f, "timeout waiting for OAuth authorization callback"),
            Self::RequestReadFailed(err) => write!(f, "failed to read HTTP request: {err}"),
            Self::InvalidRequest(s) => write!(f, "{s}"),
            Self::OAuthError(s) => write!(f, "Oauth Error: {s}"),
            Self::UrlParseFailed(err) => write!(f, "failed to parse callback URL: {err}"),
            Self::MissingQueryParam { param } => {
                write!(f, "missing required query parameter: '{param}'")
            }
        }
    }
}

impl std::error::Error for ServerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BindFailed { source, .. } => Some(source),
            Self::RequestReadFailed(err) => Some(err),
            Self::UrlParseFailed(err) => Some(err),
            _ => None,
        }
    }
}
