//!```ignore
//! use tokio::net::TcpListener;
//! use twitch_oauth::{pkce::Pkce, Token, TwitchOauth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), twitch_oauth::Error> {
//!     let (pkce_challenge, code_verifier) = Pkce::new_sha256().unwrap();
//!     // duration 10 sec
//!     let oauth = TwitchOauth::new("client_id", "client_secret", pkce_challenge, 10);
//!
//!     let auth_url = oauth.auth_request_url("chat:read");
//!
//!     // only can bind 3000
//!     let listener = TcpListener::bind("127.0.0.1:3000")
//!         .await
//!         .expect("Failed already bind 3000");
//!
//!     println!("{}", auth_url);
//!
//!     let (code, state) = oauth.oauth_server_sync(listener).await?;
//!
//!     code_verifier(state).unwrap();
//!
//!     let token = oauth.get_token_json(code).await?;
//!
//!     Ok(())
//! }
//! ```
use std::{collections::HashMap, io::Write, time::Duration};

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

pub mod pkce;

mod util;
pub use util::*;

pub struct TwitchOauth<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    duration: u64,
    state: &'a str,
    oauth_url: &'a str,
    token_url: &'a str,
    pub redirect_url: &'a str,
}

impl<'a> TwitchOauth<'a> {
    pub fn new(
        client_id: &'a str,
        client_secret: &'a str,
        state: &'a str,
        duration: u64,
    ) -> TwitchOauth<'a> {
        TwitchOauth {
            client_id,
            client_secret,
            duration,
            state,
            oauth_url: "https://id.twitch.tv/oauth2/authorize",
            redirect_url: "http://localhost:3000",
            token_url: "https://id.twitch.tv/oauth2/token",
        }
    }

    pub fn auth_request_url(&self, scope: &'a str) -> String {
        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            self.oauth_url, self.client_id, self.redirect_url, scope, self.state
        )
    }

    /// code, rev_state
    pub async fn oauth_server_sync(
        &self,
        listener: tokio::net::TcpListener,
    ) -> Result<(String, String), Error> {
        tokio::select! {
            stream = listener.accept()=>{
                match stream {
                    Ok((mut stream, _)) =>  {
                        let code;
                        let rev_state;
                        {
                            let mut reader = tokio::io::BufReader::new(&mut stream);
                            let mut request_line = String::new();
                            reader.read_line(&mut request_line).await?;

                            let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                            let url = url::Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                            rev_state = query_find(&url, "state");

                            let is_error = url.query_pairs().find(|pair| {
                                let (k, _) = pair;
                                k == "error"
                            });

                            if let Some((_,value))= is_error {
                                let error_description = query_find(&url,"error_description");

                                eprintln!("Error response : {}",value);
                                return Err(
                                    Error::GetAuthCode(AuthorizeError { error:value.into_owned(), error_description , state:rev_state  })
                                )
                            }
                            code = query_find(&url, "code");
                        }
                        let message = "close this page";
                        let response = format!(
                            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                            message.len(),
                            message
                        );
                        stream.write_all(response.as_bytes()).await?;

                        Ok((code,rev_state))
                    },
                Err(e)=>{Err(Error::IO(e))}
                }
            }
            _= async {
                let mut interval = tokio::time::interval(Duration::from_secs(1));
                let mut a:i64 = self.duration.try_into().unwrap();
                loop {
                    interval.tick().await;
                    print!("\rtimeout : {}s", a);
                    std::io::stdout().flush().unwrap();
                    a -= 1;
                }
            }=>{
                Err(Error::IO(std::io::ErrorKind::TimedOut.into()))
            }
            _= tokio::time::sleep(Duration::from_secs(self.duration)) =>{
                Err(Error::IO(std::io::ErrorKind::TimedOut.into()))
            }
        }
    }

    pub async fn get_token(&self, code: &str) -> Result<reqwest::Response, Error> {
        let reqwest_client = reqwest::Client::new();

        let mut params = HashMap::new();
        params.insert("client_id", self.client_id);
        params.insert("client_secret", self.client_secret);
        params.insert("code", code);
        params.insert("grant_type", "authorization_code");
        params.insert("redirect_uri", self.redirect_url);

        let result = reqwest_client
            .post(self.token_url)
            .form(&params)
            .send()
            .await?;

        Ok(result)
    }

    pub async fn get_token_json(self, code: &str) -> Result<Token, Error> {
        let result = self.get_token(code).await.expect("Failed get token");

        let status = result.status();
        match status {
            StatusCode::OK => {
                let result: Token = result.json().await?;
                Ok(result)
            }
            StatusCode::FORBIDDEN => {
                let result: FailToken = result.json().await?;
                Err(Error::GetOauthToken(result))
            }
            _ => {
                let result: FailToken = result.json().await?;
                Err(Error::GetOauthToken(result))
            }
        }
    }
}

fn query_find(url: &url::Url, key: &str) -> String {
    let result_pair = url
        .query_pairs()
        .find(|pair| {
            let (k, _) = pair;
            k == key
        })
        .unwrap();

    let (_, value) = result_pair;
    value.into_owned()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: Vec<String>,
    pub token_type: String,
}
