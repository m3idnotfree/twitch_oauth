# twitch_oauth_token

[![crates.io](https://img.shields.io/crates/v/twitch_oauth_token.svg)](https://crates.io/crates/twitch_oauth_token)
[![Documentation](https://docs.rs/twitch_oauth_token/badge.svg)](https://docs.rs/twitch_oauth_token)
[![license: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/m3idnotfree/twitch_oauth/blob/main/LICENSE-MIT)

A Rust library for Twitch OAuth 2.0 authentication with compile-time safety and comprehensive scope support.

- **Token management** - Refresh, validate, and revoke tokens
- **Type-safe OAuth flows** - Compile-time prevention of invalid operations using the type-state pattern
- **Stateless CSRF protection** - Cryptographically secure state validation without server-side storage
- **Full Twitch scope support** - All [Twitch API scopes](https://dev.twitch.tv/docs/authentication/scopes/) with convenient helper methods
- **Pre-configured HTTP client** - Includes an optimized authentication client preset from [asknothingx2-util](https://docs.rs/asknothingx2-util/latest/asknothingx2_util/api/preset/index.html)
- **Twitch mock API support** - Built-in support for the [Twitch CLI mock API](https://dev.twitch.tv/docs/cli/mock-api-command/) for testing and certification

## Installation

```toml
[dependencies]
twitch_oauth_token = "2"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### App access token

```rust
use twitch_oauth_token::TwitchOauth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oauth = TwitchOauth::new("client_id", "client_secret");

    let resp = oauth.app_access_token().await?;
    let token = resp.app_token().await?;

    println!("App token: {}", token.access_token.secret());
    println!("Expires in: {} seconds", token.expires_in);

    Ok(())
}
```

### User access token

```rust
use std::str::FromStr;
use twitch_oauth_token::{scope::ChatScopes, RedirectUrl, TwitchOauth};

fn main() {
    let oauth = TwitchOauth::new("client_id", "client_secret")
        .set_redirect_uri(RedirectUrl::from_str("http://example.com/auth/callback").unwrap());

    let mut auth_request = oauth.authorization_url();
    auth_request.scopes_mut().chat_api_as_user();

    // Create authorization URL for the user to visit
    let auth_url = auth_request.url();

    println!("Visit: {}", auth_url);
}
```

### Handling OAuth Callback

```rust
use twitch_oauth_token::{OAuthCallbackQuery, TwitchOauth, UserAuth};

async fn handle_callback(
    oauth: &TwitchOauth<UserAuth>,
    oauth_callback: OAuthCallbackQuery,
) -> Result<(), twitch_oauth_token::Error> {
    let response = oauth
        .user_access_token(oauth_callback.code, oauth_callback.state)
        .await?;

    let token = response.user_token().await?;

    println!("Access token: {}", token.access_token.secret());
    println!("Refresh token: {}", token.refresh_token.secret());
    println!("Scopes: {:?}", token.scope);
    println!("Expires in: {} seconds", token.expires_in);

    Ok(())
}
```

## Feature Flags

- **`oneshot-server`** - Built-in development server for handling OAuth callbacks
- **`test`** - Testing utilities and mock server support

## License

Licensed under the MIT license.
