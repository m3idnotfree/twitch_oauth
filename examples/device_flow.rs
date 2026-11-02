use std::env;

use anyhow::{Context, Result};
use twitch_oauth_token::{ClientId, DeviceAuth};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let client_id = ClientId::new(env::var("CLIENT_ID").context("CLIENT_ID is not set")?);

    let mut device_flow = DeviceAuth::new(client_id);
    device_flow.scopes_mut().chat_api();

    let resp = device_flow.device_code().await?;
    println!("Visit: {}", resp.verification_uri);

    let token = device_flow.wait_for_token(resp).await?;
    println!("token expires in {} seconds", token.expires_in);

    Ok(())
}
