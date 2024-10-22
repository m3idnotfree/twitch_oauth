//!```ignore
//! use twitch_oauth_token::{oauth_oneshot_server, types::ServerStatus, TwitchOauth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), anyhow::Error> {
//!     let mut client = TwitchOauth::default()
//!         .set_client_id("client_id")
//!         .set_client_secret("client_secret");
//!
//!     let client_credentials = client.client_credentials().await?;
//!     println!("client credentials: {client_credentials:#?}");
//!
//!     let authorize_url = client.authorize_url().add_scope("channel:bot").url();
//!
//!     println!("{authorize_url}");
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
//!             let token = client.exchange_code(rev.code.unwrap()).await?;
//!             println!("token: {:#?}", token);
//!
//!             let validate_token = client.validate_token(&token.access_token).await.unwrap();
//!             println!("validate token: {validate_token:#?}");
//!
//!             let refresh_token = client.exchange_refresh_token(&token.refresh_token).await?;
//!             println!("refresh token: {refresh_token:#?}");
//!
//!             client.revoke_token(&token.access_token).await?;
//!         }
//!     }
//!     Ok(())
//! }
//! ```
mod twitch_oauth;
pub use twitch_oauth::*;
mod oauth_oneshot_server;
pub use oauth_oneshot_server::*;
mod error;
pub use error::*;
pub mod request;
pub mod scopes;
pub mod types;

#[cfg(feature = "twitch-cli")]
pub mod twitch_cli;

pub type Result<R> = std::result::Result<R, crate::Error>;
