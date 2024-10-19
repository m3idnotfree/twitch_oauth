use http::{HeaderMap, StatusCode};
use oauth2::{AccessToken, AuthorizationCode, CsrfToken, RefreshToken};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: AccessToken,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: RefreshToken,
    pub scope: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateToken {
    pub client_id: String,
    pub login: String,
    pub scopes: Vec<String>,
    pub user_id: String,
    pub expires_in: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCredentials {
    access_token: AccessToken,
    expires_in: u64,
    token_type: String,
}

#[derive(Debug)]
pub enum ResponseType {
    Token,
    Code,
}

impl std::fmt::Display for ResponseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token => write!(f, "token"),
            Self::Code => write!(f, "code"),
        }
    }
}

impl AsRef<str> for ResponseType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Token => "token",
            Self::Code => "code",
        }
    }
}

#[derive(Debug)]
pub enum GrantType {
    ClientCredentials,
    AuthorizationCode,
    RefreshToken,
}

impl std::fmt::Display for GrantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClientCredentials => write!(f, "client_credentials"),
            Self::AuthorizationCode => write!(f, "authorization_code"),
            Self::RefreshToken => write!(f, "refresh_token"),
        }
    }
}

impl AsRef<str> for GrantType {
    fn as_ref(&self) -> &str {
        match self {
            Self::ClientCredentials => "client_credentials",
            Self::AuthorizationCode => "authorization_code",
            Self::RefreshToken => "refresh_token",
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
}

impl HttpResponse {
    pub fn json<RT>(self) -> crate::Result<RT>
    where
        RT: DeserializeOwned,
    {
        match self.status {
            StatusCode::OK => {
                let token: RT = serde_json::from_str(&self.body).unwrap();
                Ok(token)
            }
            _ => {
                let token: ErrorResponse = serde_json::from_str(&self.body).unwrap();
                Err(Error::ResponseError(token))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    pub error: Option<String>,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

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
