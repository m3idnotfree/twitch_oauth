//!```ignore
//! use tokio::net::TcpListener;
//! use twitch_oauth_token::{pkce::Pkce, Token, TwitchOauth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), twitch_oauth::Error> {
//!     let (pkce_challenge, code_verifier) = Pkce::new_sha256().unwrap();
//!     // duration 10 sec
//!     let oauth = TwitchOauth::new("client_id", "client_secret", &pkce_challenge, 10);
//!
//!     let auth_url = oauth.auth_request_url("chat:read");
//!
//!     // only can bind 3000
//!     let listener = TcpListener::bind("127.0.0.1:3000")
//!         .await
//!         .expect("Failed already bind 3000");
//!
//!     println!("{}", auth_url);
//!
//!     let (code, state) = oauth.oauth_server_sync(listener).await?;
//!
//!     code_verifier(state).unwrap();
//!
//!     let token = oauth.get_token_json(&code).await?;
//!
//!     Ok(())
//! }
//! ```

// mod twitch_oauth;
// pub use twitch_oauth::*;
// mod oauth_oneshot_server;
// pub use oauth_oneshot_server::*;
// mod error;
// pub use error::*;
// pub mod request;
// pub mod traits;
pub mod types;

pub type Result<R> = std::result::Result<R, crate::Error>;
