//! ```toml
//! twitch_oauth_token = { version = "3", features = ["oneshot-server"] }
//! ```
//!
//!```ignore
//! use twitch_oauth_token::{oauth_oneshot_server, scopes::Scopes, types::ServerStatus, TwitchOauth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), anyhow::Error> {
//!     let mut client = TwitchOauth::new(
//!         "client_id",
//!         "client_secret"
//!     );
//!
//!     let client_credentials = client.client_credentials().await?.json()?;
//!     println!("client credentials: {client_credentials:#?}");
//!
//!     let mut authorize_url = client.authorize_url();
//!     authorize_url.scopes_mut().push(Scopes::ChatRead);
//!
//!     println!("{}", authorize_url.url());
//!     let timeout = 60;
//!
//!     let rev = oauth_oneshot_server(
//!         client.get_addr().unwrap(),
//!         std::time::Duration::from_secs(timeout),
//!     )
//!     .await?;
//!
//!     match rev.state {
//!         ServerStatus::Shutdown => {
//!             println!("ctrl + c shutdown");
//!         }
//!         ServerStatus::Timeout => {
//!             println!("recive time out {}s", timeout);
//!         }
//!         ServerStatus::Recive => {
//!             let token = client.exchange_code(rev.code.unwrap()).await?.json()?;
//!             println!("token: {:#?}", token);
//!
//!             let validate_token = client.validate_token(&token.access_token).await?.json()?;
//!             println!("validate token: {validate_token:#?}");
//!
//!             let refresh_token = client
//!                 .exchange_refresh_token(&token.refresh_token)
//!                 .await?
//!                 .json()?;
//!             println!("refresh token: {refresh_token:#?}");
//!
//!             client.revoke_token(&token.access_token).await?;
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//! # Twitch Cli
//! ```toml
//! twitch_oauth_token = { version = "3", features = ["twitch-cli"] }
//! asknothingx2-util = { version = "0.0.7", features = ["api"] }
//! ```
//!
//!```ignore
//! use asknothingx2_util::api::api_request;
//! use twitch_oauth_token::{twitch_cli::{get_users_info, TwitchTest}, types::Token, TwitchOauth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), anyhow::Error> {
//!     // Does not contain a user_id
//!     // When first run twitch mock-api generate
//!     //copy user_id
//!     let users = get_users_info(None).await?;
//!     let user = users.data.first().unwrap();
//!
//!     let client = TwitchOauth::new(user.ID.as_str(), user.Secret.as_str()).test_init(None);
//!
//!     let mut test_user = client.get_mock_access_token("user_id");
//!     test_user.scopes_mut().push(Scopes::ChannelReadPolls);
//!
//!     let token = api_request(test_user).await;
//!     let token: Token = token.unwrap().json().await.unwrap();
//!     println!("{:#?}", token);
//!
//!     Ok(())
//! }
//!```
mod error;
pub use error::*;

mod twitch_oauth;
pub use twitch_oauth::*;

pub mod request;
pub mod scopes;
pub mod types;

#[cfg(feature = "oneshot-server")]
mod oauth_oneshot_server;
#[cfg(feature = "oneshot-server")]
pub use oauth_oneshot_server::*;

#[cfg(feature = "twitch-cli")]
pub mod twitch_cli;

pub type Result<R> = std::result::Result<R, crate::Error>;
