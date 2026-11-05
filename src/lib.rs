//! Twitch Authentication Library
//!
//! ## Grant Flows
//!
//! - [`Client<AppAuth>`]: Client credentials grant flow
//! - [`Client<UserAuth>`]: Authorization code grant flow, created with [`Client::with_redirect_uri`]
//! - [`DeviceAuth`]: Device code grant flow
//!
//! ## Examples
//!
//! ### Client Credentials Grant Flow
//!
//! ```no_run
//! use twitch_oauth_token::{Client, ClientId, ClientSecret};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = Client::new(
//!         ClientId::new("id".to_string()),
//!         ClientSecret::new("secret".to_string())
//!     );
//!     
//!     let token = client.app_access_token().await?;
//!     
//!     println!("Token: {}", token.access_token.secret());
//!     println!("Expires in: {} seconds", token.expires_in);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Authorization Code Grant Flow
//!
//! 1. Build the authorization URL with [`Client::authorization_url`] and save the returned [`State`]
//! 2. Check the authorization response with [`AuthorizationResponse::verify`], which returns a [`VerifiedCode`]
//! 3. Exchange code with [`Client::exchange_code`]
//!
//! ```no_run
//! use std::str::FromStr;
//! use twitch_oauth_token::{Client, ClientId, ClientSecret, RedirectUri, Scope};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = Client::new(
//!         ClientId::new("id".to_string()),
//!         ClientSecret::new("secret".to_string())
//!     )
//!     .with_redirect_uri(RedirectUri::from_str("http://localhost:3000/auth/callback")?);
//!
//!     let mut auth_request = client.authorization_url();
//!     auth_request.scopes_mut().extend([
//!         Scope::ChannelManageBroadcast,
//!         Scope::UserReadEmotes,
//!         Scope::UserWriteChat,
//!     ]);
//!
//!     let (auth_url, state) = auth_request.url();
//!     println!("Visit: {auth_url}");
//!
//!     // Save `state.secret()` in a cookie or session for the authorization response.
//!     Ok(())
//! }
//! ```
//!
//! ### Handling the Authorization Response
//!
//! ```http
//! GET /?code=...&scope=...&state=... HTTP/1.1
//! GET /?error=access_denied&error_description=...&state=... HTTP/1.1
//! ```
//!
//! Parse the query into [`AuthorizationResponse`], rebuild the saved state with [`State::new`],
//! then call [`verify`](AuthorizationResponse::verify).
//! It returns [`AuthorizationResponseError`] if the `state` doesn't match or the response has an `error`.
//!
//! ```no_run
//! use twitch_oauth_token::{
//!     AuthorizationResponse, AuthorizationResponseError, Client, ErrorCode, State, UserAuth,
//!     UserToken,
//! };
//!
//! async fn handle_redirect(
//!     client: &Client<UserAuth>,
//!     resp: AuthorizationResponse,
//!     saved_state: &State
//! ) -> anyhow::Result<UserToken> {
//!     let code = match resp.verify(saved_state) {
//!         Ok(code) => code,
//!         Err(err) if err.error_code() == Some(ErrorCode::AccessDenied) => {
//!             anyhow::bail!("the user denied access")
//!         }
//!         Err(err) => return Err(err.into())
//!     };
//!
//!     let token = client.exchange_code(&code).await?;
//!     Ok(token)
//! }
//! ```
//!
//! ### Device Code Grant Flow
//!
//! ```no_run
//! use twitch_oauth_token::{ClientId, DeviceAuth, Scope};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut device_flow = DeviceAuth::new(ClientId::new("id".to_string()));
//!     device_flow
//!         .scopes_mut()
//!         .push(Scope::UserReadEmotes)
//!         .push(Scope::UserWriteChat);
//!
//!     let resp = device_flow.device_code().await?;
//!     println!("Visit: {}", resp.verification_uri);
//!
//!     let token = device_flow.wait_for_token(resp).await?;
//!     println!("Token: {}", token.access_token.secret());
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Token Management
//!
//! ### Refresh
//!
//! Returns [`UserToken`].
//!
//! ```
//! # use twitch_oauth_token::{Error, RefreshToken, Client};
//! # async fn run(client: Client, refresh_token: RefreshToken) -> Result<(), Error> {
//! let new_token = client.refresh_access_token(&refresh_token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Validation
//!
//! Returns [`TokenInfo`].
//!
//! ```
//! # use twitch_oauth_token::{AccessToken, Error, Client};
//! # async fn run(client: Client, access_token: AccessToken) -> Result<(), Error> {
//! let token_info = client.validate_access_token(&access_token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Revocation
//!
//! ```
//! # use twitch_oauth_token::{AccessToken, Error, Client};
//! # async fn run(client: Client, access_token: AccessToken) -> Result<(), Error> {
//! client.revoke_access_token(&access_token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Security Features
//!
//! ### CSRF Protection
//!
//! [`AuthorizationRequest::url`] generates a random `state` for each authorization request.
//!
//! - Save `state.secret()` in a cookie or session before sending the user to Twitch
//! - [`AuthorizationResponse::verify`] compares it with the `state` in the authorization response
//! - [`Client::exchange_code`] only accepts the [`VerifiedCode`] that [`verify`](AuthorizationResponse::verify) returns
//!
//! ### Secure
//!
//!  Default HTTP client
//!
//! - Enforces HTTPS
//! - Does not follow redirects
//! - Uses TLS 1.2+ with certificate validation
//! - Has a 10s connect timeout and a 60s request timeout
//!
//! ## Error Handling
//!
//! Use [`Error::kind`]. [`AuthorizationResponse::verify`] returns [`AuthorizationResponseError`] instead.
//!
//! ```
//! # use twitch_oauth_token::{AppAuth, ErrorKind, Client};
//! # async fn run(client: Client<AppAuth>) {
//! match client.app_access_token().await {
//!     Ok(app_token) => { /* success */ }
//!     Err(e) => match e.kind() {
//!         ErrorKind::Api => {
//!             eprintln!("{:?} {:?}", e.status(), e.message());
//!         },
//!         ErrorKind::Request if e.is_timeout() => { /* */ },
//!         ErrorKind::Request => { /* */ },
//!         _ => eprintln!("{e}")
//!     }
//! }
//! # }
//! ```
//!
//! ## Mock API
//!
//! For local testing with Twitch Mock API, see [`mock_api`] (requires `mock-api` feature).
#![cfg_attr(docsrs, feature(doc_cfg))]

mod client;
mod device;
mod error;
mod request;
mod tokens;
mod types;

pub use client::{AppAuth, Client, ClientBuilder, OauthFlow, UserAuth};
pub use device::{DeviceAuth, DeviceAuthBuilder, DeviceCodeResponse};
pub use error::{Error, ErrorKind};
pub use request::{AuthorizationRequest, validate_access_token};
pub use tokens::{AppToken, TokenInfo, UserToken};

#[cfg(feature = "mock-api")]
pub mod mock_api;

// Re-export
pub use asknothingx2_oauth::{
    AccessToken, AuthorizationCode, AuthorizationResponse, AuthorizationResponseError, ClientId,
    ClientSecret, DeviceCode, ErrorCode, RedirectUri, RefreshToken, State, VerifiedCode,
};
pub use twitch_thunder::{ParseScopeError, Scope, Scopes};

static HTTP_CLIENT: std::sync::LazyLock<reqwest::Client> = std::sync::LazyLock::new(|| {
    asknothingx2_http::config::preset::authentication(
        reqwest::Client::builder(),
        http::HeaderValue::from_static(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        )),
    )
    .build()
    .expect("failed to build default http client")
});
