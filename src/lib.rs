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
use serde::{Deserialize, Serialize};

mod v2;
pub use v2::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: Vec<String>,
    pub token_type: String,
}
