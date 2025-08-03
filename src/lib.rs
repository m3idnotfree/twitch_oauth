#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod csrf;
pub mod response;
pub mod scope;
pub mod types;

mod error;
mod oauth;
mod request;
mod tokens;

pub use error::{Error, Kind};
pub use oauth::{client, AppAuth, TwitchOauth, UserAuth};
pub use request::{
    validate_access_token, AuthrozationRequest, ClientCredentialsRequest, CodeTokenRequest,
    RefreshRequest, RevokeRequest, ValidateRequest,
};
pub use tokens::{AppToken, UserToken, ValidateToken};

#[cfg(feature = "oneshot-server")]
mod oneshot_server;
#[cfg(feature = "oneshot-server")]
pub use oneshot_server::{oneshot_server, ServerError};

#[cfg(feature = "test")]
pub mod test_oauth;

// Re-export
pub use asknothingx2_util::oauth::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, RefreshToken,
    RevocationUrl, TokenUrl, ValidateUrl,
};
