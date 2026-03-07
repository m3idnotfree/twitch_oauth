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
//! **App Authentication** ([`TwitchOauth<AppAuth>`])
//! - For server-to-server API calls that don't require user context
//! - Access public data (streams, games, public user info)
//! - No user interaction or redirect URI required
//! - Use [`TwitchOauth::app_access_token()`] to get tokens
//!
//! **User Authentication** ([`TwitchOauth<UserAuth>`])  
//! - For applications that need to act on behalf of users
//! - Access user-specific data (follows, subscriptions, chat)
//! - Requires redirect URI and user consent flow
//! - Use [`TwitchOauth::authorization_url()`] and [`TwitchOauth::exchange_code()`]
//!
//! **Device Authentication** ([`DeviceAuth`])
//! - For desktop apps, CLI tools, and devices that cannot store a client secret
//! - Does not require a client secret or redirect URI
//! - Use [`TwitchOauth::device_auth()`], [`DeviceAuth::request()`], and [`DeviceAuth::poll()`]
//!
//! ### Type Safety
//!
//! The library uses Rust's type system to prevent common OAuth mistakes:
//! - [`TwitchOauth<AppAuth>`] only allows app authentication operations
//! - [`TwitchOauth<UserAuth>`] only allows user authentication operations  
//! - No way to accidentally call user methods without a redirect URI
//!
//! ## Examples
//!
//! ### App Authentication Flow
//!
//! ```rust,no_run
//! use twitch_oauth_token::TwitchOauth;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let oauth = TwitchOauth::new("client_id", "client_secret");
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
//! ```rust,no_run
//! use std::str::FromStr;
//! use twitch_oauth_token::{
//!     scope::{ChannelScopes, ChatScopes},
//!     AuthCallback,
//!     RedirectUrl, TwitchOauth,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let oauth = TwitchOauth::new("your_client_id", "your_client_secret")
//!         .with_redirect_uri(RedirectUrl::from_str("http://localhost:3000/auth/callback")?);
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
//! ```rust
//! use twitch_oauth_token::{AuthCallback, TwitchOauth, UserAuth};
//!
//! async fn handle_oauth_callback(
//!     oauth: &TwitchOauth<UserAuth>,
//!     query_params: AuthCallback,
//! ) -> Result<(), twitch_oauth_token::Error> {
//!     let token = oauth
//!         .exchange_code(query_params.code, query_params.state)
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
//! ```rust,no_run
//! use twitch_oauth_token::{scope::ChatScopes, ClientId, TwitchOauth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut device_flow = TwitchOauth::device_auth(ClientId::from("your_client_id"));
//!     device_flow.scopes_mut()
//!         .send_chat_message()
//!         .get_channel_emotes();
//!
//!     let resp = device_flow.request().await?;
//!     println!("Visit: {}", resp.verification_uri);
//!
//!     let token = device_flow.poll(resp).await?;
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
//! ```rust
//! use twitch_oauth_token::scope::{
//!     ChannelScopes, ChatScopes, ModerationScopes,
//!     SubscriptionScopes, UserScopes
//! };
//! # use twitch_oauth_token::{TwitchOauth, UserAuth};
//!
//! # fn run(oauth: TwitchOauth<UserAuth>) {
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
//! ```rust
//! # use twitch_oauth_token::{Error, RefreshToken, TwitchOauth};
//! # async fn run(oauth: TwitchOauth, refresh_token: RefreshToken) -> Result<(), Error> {
//! let new_token = oauth.refresh_access_token(refresh_token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Validation
//!
//! Returns [`TokenInfo`].
//!
//! ```rust
//! # use twitch_oauth_token::{AccessToken, Error, TwitchOauth};
//! # async fn run(oauth: TwitchOauth, access_token: AccessToken) -> Result<(), Error> {
//! let token_info = oauth.validate_access_token(&access_token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Revocation
//!
//! ```rust
//! # use twitch_oauth_token::{AccessToken, Error, TwitchOauth};
//! # async fn run(oauth: TwitchOauth, access_token: AccessToken) -> Result<(), Error> {
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
//! - By default, each [`TwitchOauth`] instance generates a random secret key
//! - For load-balanced or clustered environments, use [`TwitchOauth<UserAuth>::with_secret_key`] to share the same secret across all instances
//! - Without a shared secret key, tokens generated on one server will fail validation on another
//!
//! #### CSRF Configuration
//!
//! You can customize CSRF token validation behavior:
//!
//! ```rust
//! use twitch_oauth_token::{csrf::CsrfConfig, TwitchOauth};
//!
//! // Default: 30 minutes expiry, no clock skew tolerance
//! let oauth = TwitchOauth::new("client_id", "client_secret");
//!
//! // Custom: 10 minutes expiry, 60 seconds clock skew tolerance
//! let oauth = TwitchOauth::new("client_id", "client_secret")
//!     .with_csrf_config(CsrfConfig::new(60, 600));
//!
//! // Strict: 5 minutes expiry, no clock skew tolerance
//! let oauth = TwitchOauth::new("client_id", "client_secret")
//!     .with_csrf_config(CsrfConfig::default().with_max_age(300));
//! ```
//!
//! **When to customize:**
//! - **High-security apps**: Shorter expiry times (300-900 seconds)
//! - **Mobile apps**: Clock skew tolerance (30-60 seconds) for device time differences
//! - **Server clusters**: Clock skew tolerance for distributed systems and shared secret key via  [`TwitchOauth<UserAuth>::with_secret_key`]
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
//! The library provides comprehensive error types:
//!
//! ```rust
//! # use twitch_oauth_token::{TwitchOauth, AppAuth};
//!
//! # async fn run(oauth: TwitchOauth<AppAuth>) {
//!
//! match oauth.app_access_token().await {
//!     Ok(response) => { /* success */ }
//!     Err(e) => {
//!         // Network/HTTP errors (connection issues, timeouts, DNS failures)
//!         if e.is_request_error() {
//!             eprintln!("Network error: {}", e);
//!             // Common causes:
//!             // - No internet connection
//!             // - Twitch API is down
//!             // - Firewall blocking requests
//!             // - DNS resolution failure
//!
//!         // OAuth-specific errors (invalid credentials, CSRF mismatch)
//!         } else if e.is_oauth_error() {
//!             eprintln!("OAuth error: {}", e);
//!             // Common causes:
//!             // - Invalid client_id or client_secret
//!             // - CSRF token validation failed
//!             // - Authorization code expired or invalid
//!             // - Redirect URI mismatch
//!
//!         // Device code flow errors (expired code, invalid device code)
//!         } else if e.is_device_code_error() {
//!             eprintln!("Device code error: {}", e);
//!             // Common causes:
//!             // - Device code expired (user took too long)
//!
//!         // JSON deserialization errors (response doesn't match expected structure)
//!         } else if e.is_decode() {
//!             eprintln!("Deserialization error: {}", e);
//!             if let Some(raw) = e.raw() {
//!                 eprintln!("Raw response body: {}", raw);
//!             }
//!             // Common causes:
//!             // - Twitch API response schema changed
//!             // - Missing or unexpected fields in JSON
//!             // - Type mismatch (expected number, got string)
//!             // - Invalid data format
//!         }
//!     }
//! }
//! # }
//! ```
//!
//! ## Development Server
//!
//! For local development, use the built-in oneshot server to handle OAuth callbacks:
//!
//! ```rust,no_run
//! # #[cfg(feature = "oneshot")]
//! # {
//! use std::{str::FromStr, time::Duration};
//! use twitch_oauth_token::{oneshot, scope::ChatScopes, AuthCallback, RedirectUrl, TwitchOauth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let oauth = TwitchOauth::new("client_id", "client_secret")
//!         .with_redirect_uri(RedirectUrl::from_str("http://localhost:3000")?);
//!
//!     let mut auth_request = oauth.authorization_url();
//!     auth_request.scopes_mut().chat_api();
//!
//!     println!("Visit this URL to authorize:");
//!     println!("{}", auth_request.url());
//!     println!("Waiting for callback...");
//!
//!     let config = oneshot::Config::new()
//!         .with_port(3000)
//!         .with_duration(Duration::from_secs(120));
//!
//!     // Wait up to 2 minutes for the user to complete OAuth flow
//!     match oneshot::listen::<AuthCallback>(config).await {
//!         Ok(callback) => {
//!             let token = oauth
//!                 .exchange_code(callback.code, callback.state)
//!                 .await?;
//!             println!("Successfully got user token!");
//!         }
//!         Err(e) => {
//!             eprintln!("OAuth flow failed: {}", e);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! # }
//! ```
//!
//! ## HTTP Client Configuration
//!
//! The default HTTP client works for most applications.
//! For custom configuration, see [`client`].
//!
//! ## Testing
//!
//! For local testing with Twitch Mock API, see [`test_oauth`] (requires `test` feature).

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod client;
pub mod scope;

mod device;
mod error;
mod oauth;
mod request;
mod tokens;
mod types;

pub use device::{DeviceAuth, DeviceAuthResponse};
pub use error::Error;
pub use oauth::{AppAuth, TwitchOauth, UserAuth};
pub use request::{validate_access_token, AuthrozationRequest};
pub use scope::Scope;
pub use tokens::{AppToken, TokenInfo, UserToken};
pub use types::AuthCallback;

#[cfg(feature = "oneshot")]
pub use asknothingx2_util::oauth::oneshot;

#[cfg(feature = "test")]
pub mod test_oauth;

// Re-export
pub use asknothingx2_util::oauth::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, DeviceCode, DeviceUrl,
    RedirectUrl, RefreshToken, RevocationUrl, TokenUrl, ValidateUrl,
};

pub mod csrf {
    pub use asknothingx2_util::oauth::signed_token::{
        current_timestamp, extract_datetime, extract_timestamp, generate, generate_at_time,
        generate_secret_key, is_expired, token_age, verify, verify_at_time, verify_with_config,
        TokenConfig as CsrfConfig, TokenError,
    };
}
