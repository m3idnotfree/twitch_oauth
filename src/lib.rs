//! # Twitch OAuth Token Library
//!
//! This library provides a comprehensive solution for Twitch OAuth 2.0 authentication,
//! supporting both **app authentication** and **user authentication** flows with built-in
//! CSRF protection, connection pooling, and type safety.
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
//! - Use [`TwitchOauth::authorization_url()`] and [`TwitchOauth::user_access_token()`]
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
//!     let resp = oauth.app_access_token().await?;
//!     let token = resp.app_token().await?;
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
//! ```rust
//! use twitch_oauth_token::{
//!     scope::{ChatScopes, ChannelScopes},
//!     types::OAuthCallbackQuery,
//!     oauth_types::RedirectUrl,
//!     TwitchOauth,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Setup OAuth client with redirect URI
//!     let oauth = TwitchOauth::new("your_client_id", "your_client_secret")
//!         .set_redirect_uri(RedirectUrl::new("http://localhost:3000/auth/callback".to_string())?);
//!     
//!     // Create authorization URL with specific scopes
//!     let mut auth_request = oauth.authorization_url();
//!     auth_request.scopes_mut()
//!         .send_chat_message_as_user()
//!         .get_channel_emotes()
//!         .modify_channel_info();
//!     
//!     let auth_url = auth_request.url();
//!     println!("Visit: {}", auth_url);
//!     
//!     // In your callback handler:
//!     // let callback: OAuthCallbackQuery = /* parse from URL */;
//!     // let response = oauth.user_access_token(callback.code, callback.state).await?;
//!     // let token = response.user_token().await?;
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
//! See the `oauth_callback` function in [`oneshot_server`](oneshot_server::oneshot_server) for a complete parsing example.
//!
//! #### Processing the Callback
//! Regardless of how you extract the parameters, the token exchange process is the same:
//!
//! ```rust
//! use twitch_oauth_token::{types::OAuthCallbackQuery, TwitchOauth, UserAuth};
//!
//! async fn handle_oauth_callback(
//!     oauth: &TwitchOauth<UserAuth>,
//!     query_params: OAuthCallbackQuery,
//! ) -> Result<(), twitch_oauth_token::Error> {
//!     // Exchange authorization code for access token
//!     let response = oauth
//!         .user_access_token(query_params.code, query_params.state)
//!         .await?;
//!     
//!     let token = response.user_token().await?;
//!     
//!     println!("Access token: {}", token.access_token.secret());
//!     println!("Refresh token: {}", token.refresh_token.secret());
//!     println!("Granted scopes: {:?}", token.scope);
//!     println!("Expires in: {} seconds", token.expires_in);
//!     
//!     // Store the tokens securely for future API calls
//!     // store_user_tokens(user_id, &token).await?;
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
//!     .send_chat_message_as_user()           // Send chat messages
//!     .get_user_emotes()     // Read user's emotes
//!     .get_chatters()             // Read chatters list
//!     .send_chat_announcement(); // Send announcements
//!
//! // Channel management scopes  
//! auth_request.scopes_mut()
//!     .modify_channel_info()  // Update channel info
//!     .get_channel_followers() // Read followers
//!     .channel_ban_unban();   // Ban/unban users
//!
//! // Moderation scopes
//! auth_request.scopes_mut()
//!     .ban_user()             // Ban users
//!     .delete_chat_messages()      // Delete messages
//!     .update_automod_settings(); // Manage AutoMod
//!
//! // Or add entire API categories at once
//! auth_request.scopes_mut()
//!     .chat_api_as_user()             // All chat-related scopes
//!     .moderation_api()       // All moderation scopes
//!     .users_api();            // All user-related scopes
//! # }
//! ```
//!
//! ## Token Management
//!
//! ### Token Refresh
//! ```rust
//! # use twitch_oauth_token::{Error, oauth_types::RefreshToken, TwitchOauth};
//! # async fn run(oauth: TwitchOauth, refresh_token: RefreshToken) -> Result<(), Error> {
//! // Refresh an expired token
//! let refreshed_response = oauth.refresh_access_token(refresh_token).await?;
//! let new_token = refreshed_response.user_token().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Token Validation
//! ```rust
//! # use twitch_oauth_token::{oauth_types::AccessToken, Error, TwitchOauth};
//! # async fn run(oauth: TwitchOauth, access_token: AccessToken) -> Result<(), Error> {
//! // Validate a token and get user info
//! let validation_response = oauth.validate_access_token(&access_token).await?;
//! let user_info = validation_response.validate_token().await?;
//!
//! println!("Token belongs to user: {}", user_info.login);
//! println!("User ID: {}", user_info.user_id);
//! println!("Client ID: {}", user_info.client_id);
//! println!("Scopes: {:?}", user_info.scopes);
//! println!("Expires in: {} seconds", user_info.expires_in);
//! # Ok(())
//! # }
//! ```
//!
//! ### Token Revocation
//! ```rust
//! # use twitch_oauth_token::{oauth_types::AccessToken, Error, TwitchOauth};
//! # async fn run(oauth: TwitchOauth, access_token: AccessToken) -> Result<(), Error> {
//! // Revoke a token (e.g., on user logout)
//! oauth.revoke_access_token(&access_token).await?;
//! println!("Token revoked successfully");
//! # Ok(())
//! # }
//! ```
//!
//! ## HTTP Client Configuration
//!
//! ⚠️ **Warning:** Most users don't need custom HTTP client configuration.
//!
//! The library uses a global HTTP client with sensible defaults optimized for Twitch API usage.
//! For custom requirements, configure it once at application startup:
//!
//! See the [`Preset documentation`](https://docs.rs/asknothingx2-util/latest/asknothingx2_util/api/preset/struct.Preset.html)
//!
//! ```rust
//! use std::{str::FromStr, time::Duration};
//! use asknothingx2_util::api::HeaderName;
//! use twitch_oauth_token::{client, TwitchOauth};
//!
//! // Configure once at startup
//! client::setup(|preset| {
//!     Ok(preset
//!         .timeouts(Duration::from_secs(60), Duration::from_secs(30))
//!         .connections(10, Duration::from_secs(90))
//!         .default_headers(|headers| {
//!             headers
//!                 .accept_json()
//!                 .content_type_json()
//!                 .header_str(HeaderName::from_str("Custom-Header")?, "value")?;
//!             Ok(())
//!         })?
//!         .user_agent("MyApp/1.0"))
//! })?;
//!
//! // Now all OAuth instances use your custom configuration
//! let oauth = TwitchOauth::new("client_id", "client_secret");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Development Server
//!
//! For local development, use the built-in oneshot server to handle OAuth callbacks:
//!
//! ```rust,no_run
//! # #[cfg(feature = "oneshot-server")]
//! # {
//! use std::time::Duration;
//! use twitch_oauth_token::{oneshot_server::oneshot_server, scope::ChatScopes, oauth_types::RedirectUrl, TwitchOauth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let oauth = TwitchOauth::new("client_id", "client_secret")
//!         .set_redirect_uri(RedirectUrl::new("http://localhost:3000".to_string())?);
//!
//!     let mut auth_request = oauth.authorization_url();
//!     auth_request.scopes_mut().chat_api_as_user();
//!
//!     println!("Visit this URL to authorize:");
//!     println!("{}", auth_request.url());
//!     println!("Waiting for callback...");
//!
//!     // Wait up to 2 minutes for the user to complete OAuth flow
//!     match oneshot_server("127.0.0.1:3000", Duration::from_secs(120)).await {
//!         Ok(callback) => {
//!             let response = oauth
//!                 .user_access_token(callback.code, callback.state)
//!                 .await?;
//!             let token = response.user_token().await?;
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
//! ## Testing with Mock API
//!
//! When the `test` feature is enabled, you can use Twitch's mock API for testing:
//!
//! - Default port: 8080
//! - User authorization URL: http://localhost:8080/auth/authorize
//! - App authorization URL: http://localhost:8080/auth/token
//!
//! ```rust,no_run
//! # #[cfg(feature = "test")]
//! # {
//!  use twitch_oauth_token::{
//!      scope::ChatScopes,
//!      test_oauth::{mock_api::MockApiUnits, OauthTestExt, TestEnv},
//!      TwitchOauth,
//!  };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!      let mock_api = MockApiUnits::new();
//!      let clients = mock_api.get_clients().await.unwrap();
//!      let client = clients.data.first().unwrap();
//!     
//!     let oauth = TwitchOauth::from_credentials(client.ID.clone(), client.Secret.clone())
//!         .with_test_env(TestEnv::new());
//!
//!     // Get app token from mock API
//!     let app_token = oauth.app_access_token()
//!         .await?
//!         .app_token()
//!         .await?;
//!
//!     let users = mock_api.get_users().await?;
//!     let user = users.data.first().unwrap();
//!     // Get user token from mock API
//!     let mut user_token_request = oauth.user_access_token(&user.id);
//!     user_token_request.scopes_mut().send_chat_message_as_user();
//!     
//!     let user_token = user_token_request
//!         .send()
//!         .await?
//!         .user_token()
//!         .await?;
//!
//!     Ok(())
//! }
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
//! - No server-side storage required (stateless)
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
//!     .set_csrf_config(CsrfConfig::new(60, 600));
//!
//! // Strict: 5 minutes expiry, no clock skew tolerance
//! let oauth = TwitchOauth::new("client_id", "client_secret")
//!     .set_csrf_config(CsrfConfig::default().with_max_age(300));
//! ```
//!
//! **When to customize:**
//! - **High-security apps**: Shorter expiry times (300-900 seconds)
//! - **Mobile apps**: Clock skew tolerance (30-60 seconds) for device time differences
//! - **Server clusters**: Clock skew tolerance for distributed systems
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
//!         if e.is_network_error() {
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
//!         }
//!
//!         // Response parsing errors (malformed JSON, unexpected format)
//!         else if e.is_validation_error() {
//!             eprintln!("Validation error: {}", e);
//!             // Common causes:
//!             // - Twitch API returned unexpected response format
//!             // - Network corruption
//!             // - API version mismatch
//!         }
//!     }
//! }
//! # }
//! ```

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod csrf;
pub mod request;
pub mod response;
pub mod scope;
pub mod tokens;
pub mod types;

#[cfg(feature = "oneshot-server")]
pub mod oneshot_server;

#[cfg(feature = "test")]
pub mod test_oauth;

mod error;
mod oauth;

pub use error::Error;
pub use oauth::{client, AppAuth, TwitchOauth, UserAuth};

// Re-export
pub use asknothingx2_util::oauth as oauth_types;
