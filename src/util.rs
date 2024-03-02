use std::fmt;

use serde::{Deserialize, Serialize};

use crate::pkce;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to token deserialize {0}")]
    ParseJsonError(#[from] serde_json::Error),
    #[error("Failed to get token {0}")]
    PostTokenError(#[from] reqwest::Error),
    #[error("Failed to pkce verity {0}")]
    VerityPkceError(#[from] pkce::Error),
    #[error("Io Error Timeout{0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed Get Authorize code {0}")]
    GetAuthCodeError(AuthorizeError),
    #[error("Failed get token {0}")]
    OauthTokenError(FailToken),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct FailToken {
    pub kind: String,
    pub message: String,
}

impl fmt::Display for FailToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "kind: {} message: {} ", self.kind, self.message)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateToken {
    pub client_id: String,
    pub login: String,
    pub scopes: Vec<String>,
    pub user_id: String,
    pub expires_in: u64,
}
