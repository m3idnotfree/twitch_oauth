//! # Usage
//! ```toml
//! [dependencies]
//! twitch_oauth_token = { version = "1", features = ["oneshot-server"] }
//! url = "2"
//! anyhow = "1"
//! ```
//!```ignore
//! use std::time::Duration;
//!
//! use twitch_oauth_token::{
//!     oneshot_server,
//!     types::{ClientCredentials, Token, UsersScopes, ValidateToken},
//!     TwitchOauth, TokenError, validate_token
//! };
//! use url::Url;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut oauth = TwitchOauth::new(
//!         "client_id".to_string(),
//!         "client_secret".to_string(),
//!         Some("redirect_uri".to_string())
//!     )
//!     .expect("Failed to parse redirect URI in TwitchOauth initialization");
//!  
//!     let client_credentials: ClientCredentials = oauth.client_credentials().await?.into_json()?;
//!     println!("client credentials: {client_credentials:#?}");
//!  
//!     let (mut auth_request, csrf_token) = oauth.authorize_url()?;
//!     auth_request.scopes_mut().with_user_api();
//!  
//!     println!("{}", auth_request.url());
//!     let timeout = 60;
//!  
//!     let code_state = oneshot_server(
//!         oauth
//!             .get_redirect_uri()
//!             .unwrap_or(Url::parse("redirect_uri").unwrap()),
//!         Duration::from_secs(timeout),
//!     )
//!     .await?;
//!  
//!     let token: Token = oauth
//!         .exchange_code(code_state, csrf_token)
//!         .await?
//!         .into_json()?;
//!     println!("token: {:#?}", token);
//!  
//!     let validate_token: ValidateToken = validate_token(token.access_token.clone())
//!         .await?
//!         .into_json()?;
//!     println!("validate token: {validate_token:#?}");
//!  
//!     let refresh_token: Token = oauth
//!         .exchange_refresh_token(token.refresh_token)
//!         .await?
//!         .into_json()?;
//!     println!("refresh token: {refresh_token:#?}");
//!  
//!     let revoke_token = oauth.revoke_token(token.access_token).await?;
//!     if !revoke_token.is_success() {
//!         let token_err: TokenError = revoke_token.into_json().unwrap();
//!         println!("refresh token error: {token_err:#?}");
//!    }
//!  
//!     Ok(())
//! }
//! ```
//!
//! # Useing the Twitch CLI Mock Server
//! ```toml
//! [dependencies]
//! twitch_oauth_token = { version = "1", features = ["oauth", "test"] }
//! ```
//!```ignore
//! use twitch_oauth_token::{
//!     test_url::get_users_info,
//!     types::{PollsScopes, Token},
//!     TwitchOauth
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     // Does not contain a user_id
//!     // When first run twitch mock-api generate
//!     // copy user_id
//!     // <https://dev.twitch.tv/docs/cli/mock-api-command/#getting-an-access-token>
//!     let users_info = get_users_info(None).await.expect("Failed to connect to Twitch mock server");
//!     let user = users_info.data.first().expect("Mock server returned empty user data");
//!
//!     let test_oauth = TwitchOauth::from_credentials(
//!         user.ID.clone(),
//!         user.Secret.clone(),
//!         None
//!     )
//!     .expect("Failed to parse redirect URL in TwitchOauth initialization")
//!     .with_url(Some("port"));
//!
//!     // Getting a user access token
//!     let mut test_user = test_oauth.user_token("user_id".to_string());
//!     test_user.scopes_mut().with_polls_read();
//!
//!     let user_token = test_user
//!         .request_access_token()
//!         .await
//!         .expect("Failed to request user access token from mock server");
//!     let user_token: Token = user_token
//!         .into_json()
//!         .expect("Failed to deserialize user access token response");
//!     assert_eq!(vec![Scope::ChannelReadPolls], user_token.scope);
//!
//!     // Getting an app access token
//!     let mut app_token = test_oauth.app_token();
//!     app_token.scopes_mut().clear();
//!
//!     let app_token = app_token
//!         .request_access_token()
//!         .await
//!         .expect("Failed to request app access token from mock server");
//!     let app_token: Token = app_token
//!         .into_json()
//!         .expect("Failed to deserialize app access token response");
//!     assert!(app_token.scope.is_empty());
//! }
//! ```
//!
//! # Only Types
//! ```toml
//! [dependencies]
//! twitch_oauth_token = { version = "1.1.0", default-features = false, features = ["types"] }
//! ```
//! - Token
//! - ValidateToken: <https://dev.twitch.tv/docs/authentication/validate-tokens>
//! - ClientCredentials: <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
//! - Scope: <https://dev.twitch.tv/docs/authentication/scopes>
//! - ResponseType
//! - GrantType

#![cfg_attr(docsrs, feature(doc_cfg))]

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
pub mod test_url;

#[cfg(feature = "types")]
pub mod types;
