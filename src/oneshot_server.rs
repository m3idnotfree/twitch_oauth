use std::{io, net::SocketAddr, time::Duration};

use asknothingx2_util::oauth::{AuthorizationCode, CsrfToken};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{TcpListener, TcpStream},
    time::timeout,
};
use url::Url;

use crate::{Error, OAuthError, ServerError, ValidationError};

#[derive(Debug)]
pub struct CodeState {
    pub state: ServerStatus,
    pub code: Option<AuthorizationCode>,
    pub csrf_token: Option<CsrfToken>,
}

#[derive(Debug)]
pub enum ServerStatus {
    Recive,
    Shutdown,
    Timeout,
}

/// only support localhost
pub async fn oneshot_server(url: Url, duration: Duration) -> Result<CodeState, Error> {
    validate_host(&url)?;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", url.port().unwrap()))
        .await
        .map_err(|x| ServerError::BindAddressError(x.to_string()))?;

    // when this signal completes, start shutdown
    let mut signal = std::pin::pin!(shutdown_signal());

    tokio::select! {
        rev = timeout(duration, listener.accept()) => {
            handle_connection_result(rev).await
        },
        _ = &mut signal => {
            Ok(create_code_state(ServerStatus::Shutdown))
        },
    }
}

fn validate_host(url: &Url) -> Result<(), ValidationError> {
    let host = url.host_str().ok_or(ValidationError::MissingHost)?;
    if host != "localhost" {
        return Err(ValidationError::InvalidHost(host.to_string()));
    }
    Ok(())
}

async fn handle_connection_result(
    rev: Result<Result<(TcpStream, SocketAddr), io::Error>, tokio::time::error::Elapsed>,
) -> Result<CodeState, Error> {
    match rev {
        Ok(res) => {
            let (stream, _addr) = res.map_err(ServerError::Io)?;
            let (code, csrf_token) = code_state_parse(stream).await?;
            Ok(CodeState {
                state: ServerStatus::Recive,
                code: Some(AuthorizationCode::new(code)),
                csrf_token: Some(CsrfToken::new(csrf_token)),
            })
        }
        Err(_) => Ok(create_code_state(ServerStatus::Timeout)),
    }
}

fn create_code_state(status: ServerStatus) -> CodeState {
    CodeState {
        state: status,
        code: None,
        csrf_token: None,
    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

//async fn code_state_parse(mut stream: TcpStream) -> crate::Result<(String, String)> {
async fn code_state_parse(mut stream: TcpStream) -> Result<(String, String), Error> {
    let mut reader = BufReader::new(&mut stream);

    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .await
        .map_err(ServerError::Io)?;

    // "GET /?code=...&scope=...&state=.... HTTP/1.1\r\n"
    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
    let url = Url::parse(&("http://localhost".to_string() + redirect_url))
        .map_err(ValidationError::UrlParse)?;

    let code = url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, code)| code.into_owned())
        .ok_or(OAuthError::MissingQueryParam("code".to_string()))?;

    let state = url
        .query_pairs()
        .find(|(key, _)| key == "state")
        .map(|(_, state)| state.into_owned())
        .ok_or(OAuthError::MissingQueryParam("state".to_string()))?;

    Ok((code, state))
}
