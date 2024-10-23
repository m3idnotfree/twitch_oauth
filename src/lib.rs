//! features --oneshot-server
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
