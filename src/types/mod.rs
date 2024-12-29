use std::{collections::HashSet, fmt::Debug};

use asknothingx2_util::oauth::{AccessToken, AuthorizationCode, CsrfToken, RefreshToken};
use serde::{Deserialize, Serialize};

mod scopes;
pub use scopes::{Scopes, ScopesMut};

pub fn scopes_mut(scopes: &mut HashSet<Scopes>) -> ScopesMut<'_> {
    scopes::new(scopes)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: AccessToken,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: RefreshToken,
    pub scope: Vec<Scopes>,
}

impl Token {
    pub fn is_scope_empty(&self) -> bool {
        if self.scope.is_empty() {
            return true;
        }
        if self.scope.len() > 1 {
            return false;
        }

        *self.scope.first().unwrap() == Scopes::EmptyString
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateToken {
    pub client_id: String,
    pub login: String,
    pub scopes: Vec<Scopes>,
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

impl ResponseType {
    pub fn as_str(&self) -> &str {
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

impl GrantType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClientCredentials => "client_credentials",
            Self::AuthorizationCode => "authorization_code",
            Self::RefreshToken => "refresh_token",
            #[cfg(feature = "test")]
            Self::UserToken => "user_token",
        }
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