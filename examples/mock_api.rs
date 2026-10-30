use std::env;

use anyhow::{Context, Result};
use twitch_oauth_token::{
    Client,
    mock_api::{MockApiLayer, MockUnits},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let port: u16 = env::var("MOCK_API_PORT")
        .context("MOCK_API_PORT environment variable not set")?
        .parse()?;

    let units = MockUnits::localhost(port);
    let client = units
        .clients()
        .await?
        .into_iter()
        .next()
        .context("no mock clients, run `twitch mock-api generate`")?;
    let user = units
        .users()
        .await?
        .into_iter()
        .next()
        .context("no mock users, run `twitch mock-api generate`")?;

    let oauth = Client::builder(client.id, client.secret)
        .layer(MockApiLayer::localhost(port))
        .http_client(reqwest::Client::new());

    let app_token = oauth.app_access_token().await?;
    println!("{app_token:?}");

    let mut user_token_request = oauth.mock_user_token(&user.id);
    user_token_request.scopes_mut().update_user();

    let user_token = user_token_request.send().await?;
    println!("{user_token:?}");

    Ok(())
}
