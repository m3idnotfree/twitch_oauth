# twitch_oauth_token

[![CI](https://github.com/m3idnotfree/twitch_oauth/workflows/CI/badge.svg)](https://github.com/m3idnotfree/twitch_oauth/actions)
[![crates.io](https://img.shields.io/crates/v/twitch_oauth_token.svg)](https://crates.io/crates/twitch_oauth_token)
[![Documentation](https://docs.rs/twitch_oauth_token/badge.svg)](https://docs.rs/twitch_oauth_token)
[![license: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/m3idnotfree/twitch_oauth/blob/main/LICENSE-MIT)

A Rust client for Twitch OAuth 2.0.

- Refresh, validate, and revoke tokens
- Separate client types for each grant flow
- `exchange_code` accepts only a code that passed `AuthorizationResponse::verify`
- Helper methods for all [Twitch API scopes](https://dev.twitch.tv/docs/authentication/scopes/)
- [Twitch CLI mock API](https://dev.twitch.tv/docs/cli/mock-api-command/) support

## Quick Start

### Client credentials grant flow

```rust
use twitch_oauth_token::{Client, ClientId, ClientSecret};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oauth = Client::new(
        ClientId::new("client_id".to_string()),
        ClientSecret::new("client_secret".to_string())
    );

    let token = oauth.app_access_token().await?;

    println!("App token: {}", token.access_token.secret());
    println!("Expires in: {} seconds", token.expires_in);

    Ok(())
}
```

### Authorization code grant flow

```rust
use std::str::FromStr;
use twitch_oauth_token::{Client, ClientId, ClientSecret, RedirectUri};

fn main() {
    let oauth = Client::new(
        ClientId::new("client_id".to_string()),
        ClientSecret::new("client_secret".to_string())
    )
    .with_redirect_uri(RedirectUri::from_str("http://example.com/auth/callback").unwrap());

    let mut auth_request = oauth.authorization_url();
    auth_request.scopes_mut().chat_api();

    let (auth_url, state) = auth_request.url();

    println!("Visit: {}", auth_url);
}
```

### Handling the authorization response

Twitch sends the authorization response to your redirect URI.
Rebuild the saved state with `State::new` and verify the response to get a `VerifiedCode`.

```rust
use twitch_oauth_token::{AuthorizationResponse, Client, State, UserAuth};

async fn handle_authorization_response(
    oauth: &Client<UserAuth>,
    response: AuthorizationResponse,
    saved_state: &State,
) -> Result<(), Box<dyn std::error::Error>> {
    let code = response.verify(saved_state)?;
    let token = oauth.exchange_code(&code).await?;

    println!("Access token: {}", token.access_token.secret());
    println!("Refresh token: {}", token.refresh_token.secret());
    println!("Scopes: {:?}", token.scope);
    println!("Expires in: {} seconds", token.expires_in);

    Ok(())
}
```

### Device code grant flow

```rust
use twitch_oauth_token::{ClientId, DeviceAuth};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut device_flow = DeviceAuth::new(ClientId::new("client_id".to_string()));
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

- **`mock-api`** - Testing utilities and mock server support
- **`tracing`** - Log events for `DeviceAuth::wait_for_token`

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license
