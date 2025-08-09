use std::{
    fmt,
    sync::{Arc, Mutex},
    time::Duration,
};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use tokio::{net::TcpListener, sync::oneshot, time::timeout};

use crate::types::OAuthCallbackQuery;

/// **Temporary OAuth Callback Server**
///
/// Creates a lightweight HTTP server that runs temporarily to capture a single OAuth callback
/// from Twitch's authorization flow. The server automatically shuts down after receiving
/// the callback or when the timeout is reached.
///
/// ## How It Works
///
/// 1. **Starts a temporary HTTP server** on the specified address
/// 2. **Waits for the OAuth callback** from Twitch (GET request with code, state, scope)
/// 3. **Captures callback parameters** and returns them to your application
/// 4. **Automatically shuts down** after receiving callback or on timeout
/// 5. **Handles cancellation** via Ctrl+C signal
///
/// ## Server Endpoints
///
/// - `GET /` - OAuth callback handler
/// - Returns JSON response: `{"message": "Authorization successful! You can close this window."}`
/// - Error responses include details in JSON format
///
/// ## Examples
///
/// ```rust
/// // Wait up to 5 minutes for the OAuth callback
/// let callback = axum_oneshot_server("127.0.0.1:3000", Duration::from_secs(300)).await?;
/// println!("Received code: {}", callback.code.secret());
/// ```
pub async fn oneshot_server(
    addr: &str,
    duration: Duration,
) -> Result<OAuthCallbackQuery, ServerError> {
    let (tx, rx) = oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));

    let app = Router::new().route("/", get(oauth_callback)).with_state(tx);

    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| ServerError::BindFailed {
            addr: addr.to_string(),
            source: e,
        })?;

    let server = axum::serve(listener, app);

    tokio::select! {
        result = rx => {
            match result {
                Ok(callback) => Ok(callback),
                Err(_) => Err(ServerError::Shutdown),
            }
        }
        _ = timeout(duration, server) => {
            Err(ServerError::Timeout)
        }
        _ = tokio::signal::ctrl_c() => {
            Err(ServerError::Shutdown)
        }
    }
}

async fn oauth_callback(
    Query(params): Query<OAuthCallbackQuery>,
    State(tx): State<Arc<Mutex<Option<oneshot::Sender<OAuthCallbackQuery>>>>>,
) -> Result<Json<CallbackResponse>, ServerError> {
    if let Some(sender) = tx.lock().unwrap().take() {
        let _ = sender.send(params);
    }

    Ok(Json(CallbackResponse {
        message: "Authorization successful! You can close this window.".to_string(),
    }))
}

#[derive(Serialize)]
struct CallbackResponse {
    message: String,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            ServerError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            ServerError::OAuthError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(serde_json::json!({
            "error": self.to_string()
        }));

        (status, body).into_response()
    }
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
