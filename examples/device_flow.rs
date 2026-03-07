use std::env;

use anyhow::{Context, Result};
use tracing_subscriber::EnvFilter;
use twitch_oauth_token::{scope::ChatScopes, ClientId, TwitchOauth};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("twitch_oauth_token=trace")),
        )
        .init();
    dotenvy::dotenv()?;

    let client_id = env::var("CLIENT_ID").context("CLIENT_ID environment variable not set")?;
    let mut device_flow = TwitchOauth::device_auth(ClientId::from(client_id));
    device_flow.scopes_mut().chat_api();

    let resp = device_flow.request().await?;
    println!("Visit: {}", resp.verification_uri);

    let token = device_flow.poll(resp).await?;
    println!("{token:?}");
    Ok(())
}
