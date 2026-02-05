use std::{env, str::FromStr, time::Duration};

use anyhow::{Context, Result};
use twitch_oauth_token::{oneshot, AuthCallback, RedirectUrl, TwitchOauth};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let client_id = env::var("CLIENT_ID").context("CLIENT_ID environment variable not set")?;
    let client_secret =
        env::var("CLIENT_SECRET").context("CLIENT_SECRET environment variable not set")?;
    let redirect_uri =
        env::var("REDIRECT_URI").context("REDIRECT_URI environment variable not set")?;
    let port: u16 = env::var("PORT")
        .context("PORT environment variable not set")?
        .parse()?;

    let oauth = TwitchOauth::new(client_id, client_secret)
        .with_redirect_uri(RedirectUrl::from_str(&redirect_uri)?);

    let auth_url = oauth.authorization_url();

    println!("{}", auth_url.url());

    let config = oneshot::Config::new()
        .with_port(port)
        .with_callback_path("/auth/callback")
        .with_duration(Duration::from_secs(10));

    let callback: AuthCallback = oneshot::listen(config).await?;

    let token = oauth.exchange_code(callback.code, callback.state).await?;

    println!("{token:?}");
    Ok(())
}
