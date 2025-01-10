use std::time::Duration;

use twitch_oauth_token::{
    oneshot_server,
    types::{ClientCredentials, Token, UsersScopes, ValidateToken},
    validate_token, TokenError, TwitchOauth,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let client_id = std::env::var("CLIENT_ID").expect("Failed read env Variable: CLIENT_ID");
    let client_sercet =
        std::env::var("CLIENT_SECRET").expect("Failed read env Variable: CLIENT_SECRET");
    let redirect_uri =
        std::env::var("REDIRECT_URI").expect("Failed read env Variable: REDIRECT_URI");

    let mut oauth = TwitchOauth::new(client_id, client_sercet, Some(redirect_uri))
        .expect("Failed to parse redirect URI in TwitchOauth initialization");

    let client_credentials: ClientCredentials = oauth.client_credentials().await?.into_json()?;
    println!("client credentials: {client_credentials:#?}");

    let (mut auth_request, csrf_token) = oauth.authorize_url()?;
    auth_request.scopes_mut().with_user_api();

    println!("{}", auth_request.url());
    let timeout = 60;

    let code_state = oneshot_server(
        oauth.get_redirect_uri().unwrap(),
        Duration::from_secs(timeout),
    )
    .await?;

    let token: Token = oauth
        .exchange_code(code_state, csrf_token)
        .await?
        .into_json()?;
    println!("token: {:#?}", token);

    let validate_token: ValidateToken = validate_token(token.access_token.clone())
        .await?
        .into_json()?;
    println!("validate token: {validate_token:#?}");

    let refresh_token: Token = oauth
        .exchange_refresh_token(token.refresh_token)
        .await?
        .into_json()?;
    println!("refresh token: {refresh_token:#?}");

    let revoke_token = oauth.revoke_token(token.access_token).await?;
    if !revoke_token.is_success() {
        let token_err: TokenError = revoke_token.into_json().unwrap();
        println!("refresh token error: {token_err:#?}");
    }

    Ok(())
}
