use std::fmt;

use serde::Deserialize;

use crate::AuthorizationCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GrantType {
    ClientCredentials,
    AuthorizationCode,
    RefreshToken,
    #[cfg(feature = "test")]
    UserToken,
}

impl fmt::Display for GrantType {
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

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: AuthorizationCode,
    pub scope: String,
    pub state: String,
}
