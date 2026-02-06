//! Global HTTP Client Management for Oauth Request
//!
//! See also [`Preset documentation`](https://docs.rs/asknothingx2-util/latest/asknothingx2_util/api/preset/struct.Preset.html)
//!
//! This module manages a single, shared HTTP client used by all TwitchOauth instances
//! in your application. This design provides several benefits:
//!
//! - **Connection pooling**: Reuses HTTP connections across OAuth operations
//! - **Consistent configuration**: Same timeouts, headers, etc. for all requests
//! - **Memory efficiency**: One client instead of many
//! - **Thread safety**: Can be safely used from multiple threads
//!
//! # Default Behavior (No Setup Required)
//!
//! If you don't call `setup()`, a default client is created automatically with:
//! - User-Agent: "twitch-oauth/1.0"
//! - Request timeout: 60s, Connect timeout: 10s
//! - Connections: 30 max per host, 90s idle timeout
//! - TLS: 1.2+ minimum, strict validation
//! - HTTPS: Enforced (HTTP blocked)
//! - HTTP/2: Required (no HTTP/1.1 fallback)
//! - Redirects: Up to 5 allowed
//! - Cookies: Not saved, Referer: Not sent (strict security)
//! - Headers: Accept JSON, no-cache control
//!
//! This works fine for most applications.
//!
//! # Basic Usage (No Setup)
//! ```no_run
//! # use twitch_oauth_token::TwitchOauth;
//! # async fn run() -> Result<(), twitch_oauth_token::Error> {
//! let oauth = TwitchOauth::new("client_id", "client_secret");
//! let token = oauth.app_access_token().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Custom Configuration
//! ```no_run
//! use std::time::Duration;
//!
//! use asknothingx2_util::api::preset;
//! use twitch_oauth_token::{client, TwitchOauth};
//!
//! # async fn run() -> Result<(), twitch_oauth_token::Error> {
//! let mut preset = preset::authentication("MyApp/1.0");
//! preset
//!     .timeouts(Duration::from_secs(60), Duration::from_secs(30))
//!     .connections(10, Duration::from_secs(90));
//!
//! preset
//!     .default_headers_mut()
//!     .accept_json()
//!     .content_type_json();
//!
//! // Configure once at startup
//! client::setup(preset.build()?)?;
//!
//! // Now all OAuth instances use your custom client
//! let oauth = TwitchOauth::new("client_id", "client_secret");
//! # Ok(())
//! # }
//!
//! ```
use std::sync::OnceLock;

use asknothingx2_util::api::preset;
use reqwest::Client;

use crate::{error, Error};

static CLIENT: OnceLock<Client> = OnceLock::new();

/// Configure the global HTTP client used for all OAuth requests
///
/// This should be called once at application startup if you need custom
/// timeouts, proxies, or other HTTP client configuration.
///
/// # Example
/// ```no_run
/// use std::time::Duration;
///
/// use asknothingx2_util::api::preset;
/// use twitch_oauth_token::client;
///
/// # fn run() -> Result<(), twitch_oauth_token::Error> {
/// let mut preset = preset::authentication("MyApp/1.0");
/// preset
///     .timeouts(Duration::from_secs(60), Duration::from_secs(30))
///     .connections(10, Duration::from_secs(90));
///
/// preset
///     .default_headers_mut()
///     .accept_json()
///     .content_type_json();
///
/// client::setup(preset.build()?)?;
/// # Ok(())
/// # }
/// ```
pub fn setup(client: reqwest::Client) -> Result<(), Error> {
    if CLIENT.get().is_some() {
        return Err(error::client_setup::already_initialized());
    }

    CLIENT
        .set(client)
        .map_err(|_| error::client_setup::already_initialized())?;

    Ok(())
}

/// Get the global HTTP client (creates default if not configured)
pub fn get() -> &'static Client {
    CLIENT.get_or_init(|| {
        preset::authentication(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .expect("failed to build default http client")
    })
}
