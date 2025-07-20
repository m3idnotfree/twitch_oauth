#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod csrf;
pub mod types;

mod error;
mod oauth;
mod request;

pub use error::{Error, Kind};
pub use oauth::{Configured, TwitchOauth, Unconfigured};
pub use request::{
    validate_token, AuthrozationRequest, ClientCredentialsRequest, CodeTokenRequest,
    IntoRequestBuilder, RefreshRequest, RevokeRequest, ValidateRequest,
};

#[cfg(feature = "oneshot-server")]
mod oneshot_server;
#[cfg(feature = "oneshot-server")]
#[cfg_attr(docsrs, doc(cfg(feature = "oneshot-server")))]
pub use oneshot_server::{oneshot_server, ServerError};

#[cfg(feature = "test")]
pub mod test_oauth;

// Re-export
pub use asknothingx2_util::oauth::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, RefreshToken,
    RevocationUrl, TokenUrl, ValidateUrl,
};
