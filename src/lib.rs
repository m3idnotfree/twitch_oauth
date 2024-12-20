//! ```toml
//! twitch_oauth_token = { version = "1", features = ["oneshot-server"] }
//! ```
//!
//!```ignore
//! use std::time::Duration;
//!
//! use twitch_oauth_token::{
//!     oauth_oneshot_server,
//!     scopes::Scopes,
//!     types::ServerStatus,
//!     TwitchOauth
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), twitch_oauth_token::Error> {
//!     let mut oauth = TwitchOauth::new(
//!         "client_id",
//!         "client_secret"
//!     );
//!
//!     let client_credentials = oauth.client_credentials().await?.json()?;
//!     println!("client credentials: {client_credentials:#?}");
//!
//!     let (mut auth_request, csrf_token) = oauth.authorize_url();
//!     auth_request.scopes_mut().push(Scopes::ChatRead);
//!
//!     println!("{}", auth_request.url());
//!     let timeout = 60;
//!
//!     let code_state = oauth_oneshot_server(
//!         oauth.redirect_url.url().clone(),
//!         Duration::from_secs(timeout),
//!     )
//!     .await?;
//!
//!     let token = oauth.exchange_code(code_state, csrf_token).await?.json()?;
//!     println!("token: {:#?}", token);
//!
//!     let validate_token = oauth
//!         .validate_token(token.access_token.clone())
//!         .await?
//!         .json()?;
//!     println!("validate token: {validate_token:#?}");
//!
//!     let refresh_token = oauth
//!         .exchange_refresh_token(token.refresh_token)
//!         .await?
//!         .json()?;
//!     println!("refresh token: {refresh_token:#?}");
//!
//!     oauth.revoke_token(token.access_token).await?;
//!
//!     Ok(())
//! }
//! ```
//! # Useing the Twitch CLI Mock Server
//! ```toml
//! twitch_oauth_token = { version = "1", features = ["test"] }
//! asknothingx2-util = { version = "0.0.11", features = ["api"] }
//! ```
//!
//!```ignore
//! use asknothingx2_util::api::api_request;
//! use twitch_oauth_token::{
//!     scopes::Scopes,
//!     test_help::{get_users_info, TwitchTest},
//!     types::Token,
//!     TwitchOauth,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), twitch_oauth_token::Error> {
//!     // Does not contain a user_id
//!     // When first run twitch mock-api generate
//!     // copy user_id
//!     let users = get_users_info(None).await?;
//!     let user = users.data.first().unwrap();
//!
//!    let mut client = TwitchOauth::new(user.ID.as_str(), user.Secret.as_str());
//!    client.with_url("http://localhost:8080/auth/authorize");
//!
//!     // Getting a user access token
//!     let mut test_user = client.get_mock_user_access_token("user_id");
//!     test_user.scopes_mut().push(Scopes::ChannelReadPolls);
//!
//!     let token = api_request(test_user).await?;
//!     let token: Token = token.json().await?;
//!     println!("{:#?}", token);
//!
//!     Ok(())
//! }
//!```
mod error;
pub use error::Error;

mod twitch_oauth;
pub use twitch_oauth::TwitchOauth;

pub mod request;
pub mod scopes;
pub mod types;

#[cfg(feature = "oneshot-server")]
mod oauth_oneshot_server;
#[cfg(feature = "oneshot-server")]
pub use oauth_oneshot_server::oauth_oneshot_server;

#[cfg(feature = "test")]
pub mod test_help;

pub type Result<R> = std::result::Result<R, crate::Error>;
