//! # Usage
//! ```toml
//! twitch_oauth_token = { version = "1.1.0", features = ["oneshot-server"] }
//! ```
//!```ignore
//! use std::time::Duration;
//!
//! use twitch_oauth_token::{oneshot_server, types::Scope, TwitchOauth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), twitch_oauth_token::Error> {
//!   let mut oauth = TwitchOauth::new("client_id", "client_secret");
//!
//!     let client_credentials = oauth.client_credentials().await?.parse_token()?;
//!     println!("client credentials: {client_credentials:#?}");
//!
//!     let (mut auth_request, csrf_token) = oauth.authorize_url();
//!     auth_request.scopes_mut().push(Scope::ChatRead);
//!
//!     println!("{}", auth_request.url());
//!     let timeout = 60;
//!
//!     let code_state = oneshot_server(
//!         oauth.redirect_url.url().clone(),
//!         Duration::from_secs(timeout),
//!     )
//!     .await?;
//!
//!     let token = oauth
//!         .exchange_code(code_state, csrf_token)
//!         .await?
//!         .parse_token()?;
//!     println!("token: {:#?}", token);
//!
//!     let validate_token = oauth
//!         .validate_token(token.access_token.clone())
//!         .await?
//!         .parse_token()?;
//!     println!("validate token: {validate_token:#?}");
//!
//!     let refresh_token = oauth
//!         .exchange_refresh_token(token.refresh_token)
//!         .await?
//!         .parse_token()?;
//!     println!("refresh token: {refresh_token:#?}");
//!
//!     oauth.revoke_token(token.access_token).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Useing the Twitch CLI Mock Server
//! ```toml
//! twitch_oauth_token = { version = "1", features = ["oauth", "test"] }
//! asknothingx2-util = { version = "0.0.12", features = ["api"] }
//! ```
//!```ignore
//! use asknothingx2_util::api::api_request;
//! use twitch_oauth_token::{
//!     test_help::{get_users_info, TwitchTest},
//!     types::{Scope, Token},
//!     TwitchOauth,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), twitch_oauth_token::Error> {
//!     // Does not contain a user_id
//!     // When first run twitch mock-api generate
//!     // copy user_id
//!     https://dev.twitch.tv/docs/cli/mock-api-command/#getting-an-access-token
//!     let users = get_users_info(None).await?;
//!     let user = users.data.first().unwrap();
//!
//!    let mut client = TwitchOauth::new(user.ID.as_str(), user.Secret.secret().as_str());
//!    client.with_url("http://localhost:8080/auth/authorize");
//!
//!     // Getting a user access token
//!     let mut test_user = client.get_mock_user_access_token("user_id");
//!     test_user.scopes_mut().push(Scope::ChannelReadPolls);
//!
//!     let token = api_request(test_user).await?;
//!     let token: Token = token.json().await?;
//!     println!("{:#?}", token);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Only Types
//! ```toml
//! twitch_oauth_token = { version = "1.1.0", default-features = false, features = ["types"] }
//! ```
//! - Token
//! - ValidateToken: https://dev.twitch.tv/docs/authentication/validate-tokens
//! - ClientCredentials: https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow
//! - Scope: https://dev.twitch.tv/docs/authentication/scopes
//! - ResponseType
//! - GrantType

#[cfg(feature = "oauth")]
mod oauth;
#[cfg(feature = "oauth")]
pub use oauth::{TokenResponse, TwitchOauth};
#[cfg(feature = "oauth")]
mod request;
#[cfg(feature = "oauth")]
pub use request::{
    AuthrozationRequest, ClientCredentialsRequest, CodeTokenRequest, RefreshRequest, RevokeRequest,
    ValidateRequest,
};

#[cfg(any(feature = "oauth", feature = "oneshot-server"))]
mod error;
#[cfg(any(feature = "oauth", feature = "oneshot-server"))]
pub use error::Error;
#[cfg(any(feature = "oauth", feature = "oneshot-server"))]
pub type Result<R> = std::result::Result<R, crate::Error>;

#[cfg(feature = "oneshot-server")]
mod oneshot_server;
#[cfg(feature = "oneshot-server")]
pub use oneshot_server::{oneshot_server, CodeState, ServerStatus};

#[cfg(feature = "test")]
pub mod test_url;

#[cfg(feature = "types")]
pub mod types;
