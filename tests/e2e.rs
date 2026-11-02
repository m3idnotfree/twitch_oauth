mod help;

use help::{ACCESS_TOKEN, CLIENT_ID, CODE, DEVICE_CODE, REFRESH_TOKEN, server, setup, swap_origin};
use twitch_oauth_token::{AuthCallback, Scope};

#[tokio::test]
async fn client_credentials_grant_flow() {
    let (server, oauth) = setup::oauth_server().await;

    server::client_credentials(&server).await;

    let token = oauth.app_access_token().await.unwrap();
    assert_eq!(ACCESS_TOKEN, token.access_token.secret());
}

#[tokio::test]
async fn authorization_code_grant_flow() {
    let (server, oauth) = setup::oauth_server().await;

    let scopes = [Scope::ChatRead, Scope::ChatEdit];
    let mut auth_url = oauth.authorization_url();
    auth_url.scopes_mut().extend(scopes);

    let url = swap_origin(auth_url.url(), &server);
    let state = server::authorization_code_grant_flow(&server, &url, &scopes).await;

    let resp = help::client()
        .get(url)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let auth_callback: AuthCallback = serde_json::from_slice(&resp).unwrap();

    assert_eq!(CODE, auth_callback.code.secret());
    assert_eq!(state, auth_callback.state);

    let token = oauth
        .exchange_code(&auth_callback.code, &auth_callback.state)
        .await
        .unwrap();

    assert_eq!(ACCESS_TOKEN, token.access_token.secret());
    assert!(token.scope.contains(&Scope::ChatRead));
    assert!(token.scope.contains(&Scope::ChatEdit));
}

#[tokio::test]
async fn refresh_access_token() {
    let (server, oauth) = setup::oauth_server().await;

    server::refresh_access_token(&server).await;

    let token = oauth
        .refresh_access_token(&twitch_oauth_token::RefreshToken::new(
            REFRESH_TOKEN.to_string(),
        ))
        .await
        .unwrap();
    assert_eq!(token.access_token.secret(), ACCESS_TOKEN);
}

#[tokio::test]
pub async fn validate_access_token() {
    let (server, oauth) = setup::oauth_server().await;

    server::validate_access_token(&server).await;

    let token = oauth
        .validate_access_token(&twitch_oauth_token::AccessToken::new(
            ACCESS_TOKEN.to_string(),
        ))
        .await
        .unwrap();
    assert_eq!(CLIENT_ID, token.client_id.to_string());
}

#[tokio::test]
pub async fn revoke_access_token() {
    let (server, oauth) = setup::oauth_server().await;

    server::revoke_access_token(&server).await;

    oauth
        .revoke_access_token(&twitch_oauth_token::AccessToken::new(
            ACCESS_TOKEN.to_string(),
        ))
        .await
        .unwrap();
}

#[tokio::test]
pub async fn device_code_grant_flow() {
    let (server, mut device) = setup::device_server().await;
    device.scopes_mut().chat_api();

    server::device_code_grant_flow(&server).await;

    let resp = device.device_code().await.unwrap();
    assert_eq!(DEVICE_CODE, resp.device_code.secret());

    let token = device.wait_for_token(resp).await.unwrap();
    assert_eq!(ACCESS_TOKEN, token.access_token.secret());
}
