mod help;

use help::{config, server, setup, token};
use twitch_oauth_token::types::OAuthCallbackQuery;

#[tokio::test]
async fn client_credentials_grant_flow() {
    let (server, oauth) = setup::oauth_server().await;

    server::client_credentials(&server).await;

    let token = oauth.app_access_token().await.unwrap();
    assert_eq!(200, token.status());

    let token = token.app_token().await.unwrap();
    assert_eq!(token::access().secret(), token.access_token.secret());
}

#[tokio::test]
async fn authorization_code_grant_flow() {
    let (server, oauth) = setup::oauth_server().await;

    let state = server::authorization_code_grant_flow(&server, &oauth, "").await;

    let auth_url = oauth.authorization_url().url();
    let resp: OAuthCallbackQuery = config::client()
        .get(auth_url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(&token::code(), resp.code.secret());
    assert_eq!(state, resp.state);

    let token = oauth
        .user_access_token(resp.code, resp.state)
        .await
        .unwrap();

    assert_eq!(200, token.status());

    let token = token.user_token().await.unwrap();
    assert_eq!(token::access().secret(), token.access_token.secret());
}

#[tokio::test]
async fn refresh_access_token() {
    let (server, oauth) = setup::oauth_server().await;

    server::refresh_access_token(&server).await;

    let token = oauth.refresh_access_token(token::refresh()).await.unwrap();
    assert_eq!(200, token.status());

    let token = token.user_token().await.unwrap();
    assert_eq!(token.access_token.secret(), token::access().secret());
}

#[tokio::test]
pub async fn validate_access_token() {
    let (server, oauth) = setup::oauth_server().await;

    server::validate_access_token(&server).await;

    let token = oauth.validate_access_token(&token::access()).await.unwrap();
    assert_eq!(200, token.status());

    let token = token.validate_token().await.unwrap();
    assert_eq!(config::client_id(), token.client_id);
}

#[tokio::test]
pub async fn revoke_access_token() {
    let (server, oauth) = setup::oauth_server().await;

    server::revoke_access_token(&server).await;

    let token = oauth.revoke_access_token(&token::access()).await.unwrap();

    assert_eq!(204, token.status());
}
