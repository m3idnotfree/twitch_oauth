use std::env;

use anyhow::{Context, Result};
use twitch_oauth_token::{Client, ClientId, ClientSecret};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let client = Client::new(
        ClientId::new(env::var("CLIENT_ID").context("CLIENT_ID is not set")?),
        ClientSecret::new(env::var("CLIENT_SECRET").context("CLIENT_SECRET is not set")?),
    );

    let token = client.app_access_token().await?;
    println!("token expires in {} seconds", token.expires_in);

    Ok(())
}
