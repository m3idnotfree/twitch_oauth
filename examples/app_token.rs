use std::{env, time::Instant};

use anyhow::{Context, Result};
use tracing::{debug, error, info};
use twitch_oauth_token::TwitchOauth;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    dotenvy::dotenv()?;

    let client_id = env::var("CLIENT_ID").context("CLIENT_ID environment variable not set")?;
    let client_secret =
        env::var("CLIENT_SECRET").context("CLIENT_SECRET environment variable not set")?;

    let start_time = Instant::now();

    info!(
        service = "twitch_oauth",
        action = "get_app_token",
        "Getting OAuth token"
    );

    let oauth = TwitchOauth::new(client_id, client_secret);

    debug!(
        service = "twitch_oauth",
        action = "get_app_token",
        "Calling Twitch API"
    );

    let token = oauth
        .app_access_token()
        .await
        .map_err(|e| {
            let error_type = classify_error(&e);

            error!(
                service = "twitch_oauth",
                action = "get_app_token",
                error_type = error_type,
                error_message = %e,
                duration_ms = start_time.elapsed().as_millis(),
                "Failed to get OAuth token"
            );

            e
        })
        .context("Failed to get Twitch app access token")?;

    debug!(
        service = "twitch_oauth",
        action = "get_app_token",
        duration_ms = start_time.elapsed().as_millis(),
        "Received API response"
    );

    info!(
        service = "twitch_oauth",
        action = "get_app_token",
        token_type = token.token_type,
        expires_in = ?token.expires_in,
        duration_ms = start_time.elapsed().as_millis(),
        "Successfully got OAuth token"
    );

    Ok(())
}

fn classify_error(e: &twitch_oauth_token::Error) -> &str {
    if e.is_network_error() {
        "network"
    } else if e.is_oauth_error() {
        "oauth"
    } else if e.is_decode() {
        "decode"
    } else {
        "unknown"
    }
}
