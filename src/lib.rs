//!```ignore
//! use twitch_oauth_token::{oauth_oneshot_server, TwitchOauth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), anyhow::Error> {
//!     let mut client = TwitchOauth::default()
//!         .set_client_id("client_id")
//!         .set_client_secret("client_secret");
//!
//!     let authorize_url = client.authorize_url().add_scope("channel:bot").url();
//!
//!     println!("{authorize_url}");
//!
//!     let rev = oauth_oneshot_server(
//!         client.get_addr().unwrap(),
//!         std::time::Duration::from_secs(60),
//!     )
//!     .await?;
//!
//!     let token = client.exchange_code_with_statuscode(rev).await?;
//!     println!("token: {:#?}", token);
//!
//!     let validate_token = client.validate_token(&token.access_token).await.unwrap();
//!     println!("validate token: {validate_token:#?}");
//!
//!     let refresh_token = client.exchange_refresh_token(&token.refresh_token).await?;
//!     println!("refresh token: {refresh_token:#?}");
//!
//!     client.revoke_token(&token.access_token).await?;
//!
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
pub mod traits;
pub mod types;

pub type Result<R> = std::result::Result<R, crate::Error>;
