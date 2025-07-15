use std::{io, net::SocketAddr, time::Duration};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{TcpListener, TcpStream},
    time::timeout,
};
use url::Url;

use crate::{
    error::{self, Error},
    AuthorizationCode,
};

#[derive(Debug)]
pub struct CodeState {
    pub state: ServerStatus,
    pub code: Option<AuthorizationCode>,
    pub csrf_token: Option<String>,
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
    let address = format!("127.0.0.1:{}", url.port().unwrap());
    let listener = TcpListener::bind(&address)
        .await
        .map_err(|e| error::server::bind(&address, e))?;

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

fn validate_host(url: &Url) -> Result<(), Error> {
    let host = url.host_str().ok_or_else(error::validation::missing_host)?;
    if host != "localhost" {
        return Err(error::validation::invalid_host(host));
    }
    Ok(())
}

async fn handle_connection_result(
    rev: Result<Result<(TcpStream, SocketAddr), io::Error>, tokio::time::error::Elapsed>,
) -> Result<CodeState, Error> {
    match rev {
        Ok(res) => {
            let (stream, _addr) = res.map_err(Error::from)?;
            let (code, csrf_token) = code_state_parse(stream).await?;
            Ok(CodeState {
                state: ServerStatus::Recive,
                code: Some(AuthorizationCode::new(code)),
                csrf_token: Some(csrf_token),
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

async fn code_state_parse(mut stream: TcpStream) -> Result<(String, String), Error> {
    let mut reader = BufReader::new(&mut stream);

    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .await
        .map_err(Error::from)?;

    // "GET /?code=...&scope=...&state=.... HTTP/1.1\r\n"
    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
    let url = Url::parse(&("http://localhost".to_string() + redirect_url)).map_err(Error::from)?;

    let code = url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, code)| code.into_owned())
        .ok_or_else(|| error::validation::missing_query_param("code"))?;

    let state = url
        .query_pairs()
        .find(|(key, _)| key == "state")
        .map(|(_, state)| state.into_owned())
        .ok_or_else(|| error::validation::missing_query_param("state"))?;

    Ok((code, state))
}
