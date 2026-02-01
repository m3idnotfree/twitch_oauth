use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::Deserialize;

use crate::AuthorizationCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ResponseType {
    // Token,
    Code,
}

impl ResponseType {
    pub fn as_str(&self) -> &str {
        match self {
            // Self::Token => "token",
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

impl Display for GrantType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
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
