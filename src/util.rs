use std::fmt;

use serde::{Deserialize, Serialize};

use crate::pkce;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to token deserialize {0}")]
    ParseJson(#[from] serde_json::Error),
    #[error("Failed to post token {0}")]
    PostToken(#[from] reqwest::Error),
    #[error("Failed to pkce verity {0}")]
    PostPkce(#[from] pkce::PkceError),
    #[error("Io Error Timeout{0}")]
    IO(#[from] std::io::Error),
    #[error("Failed Get Authorize code {0}")]
    GetAuthCode(AuthorizeError),
    #[error("Failed get token {0}")]
    GetOauthToken(FailToken),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AuthorizeError {
    pub error: String,
    pub error_description: String,
    pub state: String,
}

impl AuthorizeError {
    pub fn new<T: Into<String>>(error: T, error_description: T, state: T) -> AuthorizeError {
        AuthorizeError {
            error: error.into(),
            error_description: error_description.into(),
            state: state.into(),
        }
    }
}

impl fmt::Display for AuthorizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to get authorize code error: {}, error_description: {}, state: {}",
            self.error, self.error_description, self.state
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FailToken {
    pub message: String,
    pub status: u64,
}

impl fmt::Display for FailToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to get oauth token message: {}, status: {}",
            self.message, self.status
        )
    }
}
