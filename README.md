# twitch_oauth_token

[![CI](https://github.com/m3idnotfree/twitch_oauth/workflows/CI/badge.svg)](https://github.com/m3idnotfree/twitch_oauth/actions)
[![crates.io](https://img.shields.io/crates/v/twitch_oauth_token.svg)](https://crates.io/crates/twitch_oauth_token)
[![Documentation](https://docs.rs/twitch_oauth_token/badge.svg)](https://docs.rs/twitch_oauth_token)
[![license: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/m3idnotfree/twitch_oauth/blob/main/LICENSE-MIT)

A Rust library for Twitch OAuth 2.0 authentication with compile-time safety and comprehensive scope support.

- **Token management** - Refresh, validate, and revoke tokens
- **Type-safe OAuth flows** - Compile-time prevention of invalid operations using the type-state pattern
- **HMAC-based CSRF protection** - Cryptographically secure state validation using HMAC-SHA256 with timestamp validation
- **Full Twitch scope support** - All [Twitch API scopes](https://dev.twitch.tv/docs/authentication/scopes/) with convenient helper methods
- **Pre-configured HTTP client** - Includes an optimized authentication client preset from [asknothingx2-util](https://docs.rs/asknothingx2-util/latest/asknothingx2_util/api/preset/index.html)
- **Twitch mock API support** - Built-in support for the [Twitch CLI mock API](https://dev.twitch.tv/docs/cli/mock-api-command/) for testing and certification

## Installation

```toml
[dependencies]
twitch_oauth_token = "5"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### App access token

```rust
use twitch_oauth_token::{Client, ClientId, ClientSecret};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oauth = Client::new(
        ClientId::from("client_id"),
        ClientSecret::from("client_secret")
    );

    let token = oauth.app_access_token().await?;

    println!("App token: {}", token.access_token.secret());
    println!("Expires in: {} seconds", token.expires_in);

    Ok(())
}
```

### User access token

```rust
use std::str::FromStr;
use twitch_oauth_token::{Client, ClientId, ClientSecret, RedirectUrl};

fn main() {
    let oauth = Client::new(
        ClientId::from("client_id"),
        ClientSecret::from("client_secret")
    )
    .with_redirect_uri(RedirectUrl::from_str("http://example.com/auth/callback").unwrap());

    let mut auth_request = oauth.authorization_url();
    auth_request.scopes_mut().chat_api();

    // Create authorization URL for the user to visit
    let auth_url = auth_request.url();

    println!("Visit: {}", auth_url);
}
```

### Handling OAuth Callback

```rust
use twitch_oauth_token::{AuthCallback, Client, UserAuth};

async fn handle_callback(
    oauth: &Client<UserAuth>,
    oauth_callback: AuthCallback,
) -> Result<(), twitch_oauth_token::Error> {
    let token = oauth
        .exchange_code(oauth_callback.code, oauth_callback.state)
        .await?;

    println!("Access token: {}", token.access_token.secret());
    println!("Refresh token: {}", token.refresh_token.secret());
    println!("Scopes: {:?}", token.scope);
    println!("Expires in: {} seconds", token.expires_in);

    Ok(())
}
```

### Device code Flow

```rust
use twitch_oauth_token::{ClientId, DeviceAuth};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut device_flow = DeviceAuth::new(ClientId::from("client_id"));
    device_flow.scopes_mut().chat_api();

    let resp = device_flow.device_code().await?;
    println!("Visit: {}", resp.verification_uri);

    let token = device_flow.wait_for_token(resp).await?;
    println!("Access token: {}", token.access_token.secret());
    println!("Refresh token: {}", token.refresh_token.secret());
    println!("Scopes: {:?}", token.scope);
    println!("Expires in: {} seconds", token.expires_in);

    Ok(())
}
```

## Feature Flags

- **`oneshot`** - Built-in development server for handling OAuth callbacks
- **`mock-api`** - Testing utilities and mock server support
- **`tracing`** - Log events for `DeviceAuth::wait_for_token`

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license
