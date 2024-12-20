use std::{fmt::Debug, marker::PhantomData};

use asknothingx2_util::{
    api::StatusCode,
    oauth::{AccessToken, AuthorizationCode, CsrfToken, RefreshToken},
};
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
    pub access_token: AccessToken,
    pub expires_in: u64,
    pub token_type: String,
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
    #[cfg(feature = "test")]
    UserToken,
}

impl std::fmt::Display for GrantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClientCredentials => write!(f, "client_credentials"),
            Self::AuthorizationCode => write!(f, "authorization_code"),
            Self::RefreshToken => write!(f, "refresh_token"),
            #[cfg(feature = "test")]
            Self::UserToken => write!(f, "user_token"),
        }
    }
}

impl AsRef<str> for GrantType {
    fn as_ref(&self) -> &str {
        match self {
            Self::ClientCredentials => "client_credentials",
            Self::AuthorizationCode => "authorization_code",
            Self::RefreshToken => "refresh_token",
            #[cfg(feature = "test")]
            Self::UserToken => "user_token",
        }
    }
}

pub struct OauthResponse<RT>
where
    RT: DeserializeOwned,
{
    pub status_code: StatusCode,
    pub body: String,
    _phantom: PhantomData<RT>,
}

impl<RT> Debug for OauthResponse<RT>
where
    RT: DeserializeOwned,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OauthResponse")
            .field("status_code", &self.status_code)
            .field("body", &self.body)
            .finish()
    }
}

impl<RT> OauthResponse<RT>
where
    RT: DeserializeOwned,
{
    pub fn new(status_code: StatusCode, body: String) -> Self {
        OauthResponse {
            status_code,
            body,
            _phantom: PhantomData,
        }
    }
    pub fn json(self) -> crate::Result<RT> {
        match self.status_code {
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
