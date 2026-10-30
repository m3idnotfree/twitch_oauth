//! Twitch OAuth Token Library
//!
//! This library provides a comprehensive solution for Twitch OAuth 2.0 authentication,
//! supporting **app authentication**, **user authentication**, and **device authentication**
//! flows with built-in CSRF protection, connection pooling, and type safety.
//!
//! ## Core Concepts
//!
//! ### Authentication Flows
//!
//! **App Authentication** ([`Client<AppAuth>`])
//! - For server-to-server API calls that don't require user context
//! - Access public data (streams, games, public user info)
//! - No user interaction or redirect URI required
//! - Use [`Client::app_access_token()`] to get tokens
//!
//! **User Authentication** ([`Client<UserAuth>`])  
//! - For applications that need to act on behalf of users
//! - Access user-specific data (follows, subscriptions, chat)
//! - Requires redirect URI and user consent flow
//! - Use [`Client::authorization_url()`] and [`Client::exchange_code()`]
//!
//! **Device Authentication** ([`DeviceAuth`])
//! - For desktop apps, CLI tools, and devices that cannot store a client secret
//! - Does not require a client secret or redirect URI
//! - Use [`DeviceAuth::new`], [`DeviceAuth::device_code()`], and [`DeviceAuth::wait_for_token()`]
//!
//! ### Type Safety
//!
//! The library uses Rust's type system to prevent common OAuth mistakes:
//! - [`Client<AppAuth>`] only allows app authentication operations
//! - [`Client<UserAuth>`] only allows user authentication operations  
//! - No way to accidentally call user methods without a redirect URI
//!
//! ## Examples
//!
//! ### App Authentication Flow
//!
//! ```no_run
//! use twitch_oauth_token::{Client, ClientId, ClientSecret};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let oauth = Client::new(
//!         ClientId::from("client_id"),
//!         ClientSecret::from("client_secret")
//!     );
//!     
//!     let token = oauth.app_access_token().await?;
//!     
//!     println!("App token: {}", token.access_token.secret());
//!     println!("Expires in: {} seconds", token.expires_in);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### User Authentication Flow
//!
//! ```no_run
//! use std::str::FromStr;
//! use twitch_oauth_token::{
//!     AuthCallback, Client, ClientId,
//!     ClientSecret, RedirectUrl,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let oauth = Client::new(
//!         ClientId::from("your_client_id"),
//!         ClientSecret::from("your_client_secret")
//!     )
//!     .with_redirect_uri(RedirectUrl::from_str("http://localhost:3000/auth/callback")?);
//!
//!     let mut auth_request = oauth.authorization_url();
//!     auth_request.scopes_mut()
//!         .send_chat_message()
//!         .get_channel_emotes()
//!         .modify_channel_info();
//!
//!     let auth_url = auth_request.url();
//!     println!("Visit: {}", auth_url);
//!
//!     // In your callback handler:
//!     // let callback: AuthCallback = /* parse from URL */;
//!     // let token = oauth.exchange_code(callback.code, callback.state).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Handling OAuth Callbacks
//!
//! When the user grants access, Twitch redirects them back to your redirect URL with the authorization result.
//!
//! #### Raw HTTP Format
//! If you're not using a web framework, the callback arrives as a raw HTTP request:
//!
//! ```http
//! GET /?code=...&scope=...&state=... HTTP/1.1
//! ```
//!
//! The query parameters contain:
//! - `code`: Authorization code to exchange for tokens
//! - `state`: CSRF protection token (must match what you sent)
//! - `scope`: Space-separated scopes that were actually granted
//!
//! #### Processing the Callback
//!
//! Regardless of how you extract the parameters, the token exchange process is the same.
//! The library provides [`AuthCallback`] to parse OAuth callback parameters.
//! Use it to extract the authorization code and state
//!
//! ```
//! use twitch_oauth_token::{AuthCallback, Client, UserAuth};
//!
//! async fn handle_oauth_callback(
//!     oauth: &Client<UserAuth>,
//!     query_params: AuthCallback,
//! ) -> Result<(), twitch_oauth_token::Error> {
//!     let token = oauth
//!         .exchange_code(&query_params.code, &query_params.state)
//!         .await?;
//!     
//!     // Store the tokens securely for future API calls
//!     // store_user_tokens(user_id, &token).await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Device Authentication Flow
//!
//! For desktop apps, CLI tools, and devices that cannot store a `client_secret`.
//!
//! ```no_run
//! use twitch_oauth_token::{ClientId, DeviceAuth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut device_flow = DeviceAuth::new(ClientId::from("your_client_id"));
//!     device_flow.scopes_mut()
//!         .send_chat_message()
//!         .get_channel_emotes();
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
//! ## Working with Scopes
//!
//! The library provides type-safe scope builders organized by API category:
//!
//! ```
//! # use twitch_oauth_token::{Client, UserAuth};
//!
//! # fn run(oauth: Client<UserAuth>) {
//! let mut auth_request = oauth.authorization_url();
//!
//! // Chat-related scopes
//! auth_request.scopes_mut()
//!     .send_chat_message()        // Send chat messages
//!     .get_user_emotes()          // Read user's emotes
//!     .get_chatters()             // Read chatters list
//!     .send_chat_announcement();  // Send announcements
//!
//! // Channel management scopes  
//! auth_request.scopes_mut()
//!     .modify_channel_info()      // Update channel info
//!     .get_channel_followers()    // Read followers
//!     .channel_ban_unban();       // Ban/unban users
//!
//! // Moderation scopes
//! auth_request.scopes_mut()
//!     .ban_user()                 // Ban users
//!     .delete_chat_messages()     // Delete messages
//!     .update_automod_settings(); // Manage AutoMod
//!
//! // Or add entire API categories at once
//! auth_request.scopes_mut()
//!     .chat_api()                 // All chat-related scopes
//!     .moderation_api()           // All moderation scopes
//!     .users_api();               // All user-related scopes
//! # }
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
//! # async fn run(oauth: Client, refresh_token: RefreshToken) -> Result<(), Error> {
//! let new_token = oauth.refresh_access_token(&refresh_token).await?;
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
//! # async fn run(oauth: Client, access_token: AccessToken) -> Result<(), Error> {
//! let token_info = oauth.validate_access_token(&access_token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Revocation
//!
//! ```
//! # use twitch_oauth_token::{AccessToken, Error, Client};
//! # async fn run(oauth: Client, access_token: AccessToken) -> Result<(), Error> {
//! oauth.revoke_access_token(&access_token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Security Features
//!
//! ### CSRF Protection
//!
//! The library automatically generates and validates CSRF tokens using HMAC-SHA256:
//! - Tokens are cryptographically signed with a random secret key
//! - Automatic validation prevents CSRF attacks
//! - Configurable expiration and clock skew tolerance
//! - Self-contained validation without database storage
//!
//! **Important for multi-server deployments:**
//! - By default, each [`Client`] instance generates a random secret key
//! - For load-balanced or clustered environments, use [`ClientBuilder::csrf_key`] to share the same secret across all instances
//! - Without a shared secret key, tokens generated on one server will fail validation on another
//!
//! #### CSRF Configuration
//!
//! You can customize CSRF token validation behavior:
//!
//! ```
//! use twitch_oauth_token::{Client, ClientId, ClientSecret, csrf::Config};
//!
//! // Default: 30 minutes expiry, no clock skew tolerance
//! let oauth = Client::builder(
//!     ClientId::from("client_id"),
//!     ClientSecret::from("client_secret"),
//! )
//! .build();
//!
//! // Custom: 10 minutes expiry, 60 seconds clock skew tolerance
//! let oauth = Client::builder(
//!     ClientId::from("client_id"),
//!     ClientSecret::from("client_secret"),
//! )
//! .csrf_config(Config::new(60, 600))
//! .build();
//!
//! // Strict: 5 minutes expiry, no clock skew tolerance
//! let oauth = Client::builder(
//!     ClientId::from("client_id"),
//!     ClientSecret::from("client_secret"),
//! )
//! .csrf_config(Config::default().with_max_age(300))
//! .build();
//! ```
//!
//! **When to customize:**
//! - **High-security apps**: Shorter expiry times (300-900 seconds)
//! - **Mobile apps**: Clock skew tolerance (30-60 seconds) for device time differences
//! - **Server clusters**: Clock skew tolerance for distributed systems and shared secret key via  [`ClientBuilder::csrf_key`]
//!
//! ### Secure Defaults
//!
//! - HTTPS enforcement (HTTP requests are blocked)
//! - Secure HTTP headers (no-cache, strict security)
//! - TLS 1.2+ minimum with certificate validation
//! - HTTP/2 required (no HTTP/1.1 fallback)
//! - Connection pooling with idle timeouts
//!
//! ## Error Handling
//!
//! Use [Error::kind]
//!
//! ```
//! # use twitch_oauth_token::{AppAuth, ErrorKind, Client};
//! # async fn run(oauth: Client<AppAuth>) {
//! match oauth.app_access_token().await {
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
//! ## HTTP Client Configuration
//!
//! The default HTTP client works for most applications.
//! For custom configuration, see [`ClientBuilder::http_client`].
//!
//! ## Testing
//!
//! For local testing with Twitch Mock API, see [`mock_api`] (requires `mock-api` feature).

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod scope;

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
pub use scope::Scope;
pub use tokens::{AppToken, TokenInfo, UserToken};
pub use types::AuthCallback;

#[cfg(feature = "mock-api")]
pub mod mock_api;

// Re-export
pub use asknothingx2_util::oauth::{
    AccessToken, AuthorizationCode, ClientId, ClientSecret, DeviceCode, RedirectUrl, RefreshToken,
};

pub mod csrf {
    pub use asknothingx2_util::oauth::signed_token::{
        Error, TokenConfig as Config, generate_secret_key,
    };

    pub(crate) use asknothingx2_util::oauth::signed_token::{generate, verify_with_config};
}

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
