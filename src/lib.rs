#![cfg_attr(docsrs, feature(doc_cfg))]

pub use asknothingx2_util::oauth::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, RefreshToken,
    RevocationUrl, TokenUrl, ValidateUrl,
};

pub const APPTYPE: asknothingx2_util::api::app_type::AppType =
    asknothingx2_util::api::app_type::AppType::from_static("twitch-oauth");

pub mod csrf;

mod error;
pub use error::{Error, Kind};
#[cfg(feature = "oauth")]
mod oauth;
#[cfg(feature = "oauth")]
mod request;

#[cfg(feature = "oauth")]
pub use oauth::{TokenError, TwitchOauth};
#[cfg(feature = "oauth")]
pub use request::{
    validate_token, AuthrozationRequest, ClientCredentialsRequest, CodeTokenRequest,
    RefreshRequest, RevokeRequest, ValidateRequest,
};

#[cfg(feature = "oneshot-server")]
mod oneshot_server;
#[cfg(feature = "oneshot-server")]
#[cfg_attr(docsrs, doc(cfg(feature = "oneshot-server")))]
pub use oneshot_server::{oneshot_server, CodeState, ServerStatus};

#[cfg(feature = "test")]
pub mod test_oauth;

#[cfg(feature = "types")]
pub mod types;
