mod help;

use help::{
    ACCESS_TOKEN, CLIENT_ID, DEVICE_CODE, access_token, refresh_token, server, setup, swap_origin,
};
use twitch_oauth_token::{AuthorizationResponse, Scope, State};

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

    let (auth_url, state) = auth_url.url();
    let url = swap_origin(auth_url, &server);
    let url_state = server::authorization_code_grant_flow(&server, &url, &scopes).await;

    assert_eq!(state, url_state);

    let saved_state = state.secret().to_string();

    let resp = help::client()
        .get(url)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let response: AuthorizationResponse = serde_json::from_slice(&resp).unwrap();

    let verified = response.verify(&State::new(saved_state)).unwrap();

    let token = oauth.exchange_code(&verified).await.unwrap();

    assert_eq!(ACCESS_TOKEN, token.access_token.secret());
    assert!(token.scope.contains(&Scope::ChatRead));
    assert!(token.scope.contains(&Scope::ChatEdit));
}

#[tokio::test]
async fn refresh_access_token() {
    let (server, oauth) = setup::oauth_server().await;

    server::refresh_access_token(&server).await;

    let token = oauth.refresh_access_token(&refresh_token()).await.unwrap();
    assert_eq!(token.access_token.secret(), ACCESS_TOKEN);
}

#[tokio::test]
pub async fn validate_access_token() {
    let (server, oauth) = setup::oauth_server().await;

    server::validate_access_token(&server).await;

    let token = oauth.validate_access_token(&access_token()).await.unwrap();
    assert_eq!(CLIENT_ID, token.client_id.to_string());
}

#[tokio::test]
pub async fn revoke_access_token() {
    let (server, oauth) = setup::oauth_server().await;

    server::revoke_access_token(&server).await;

    oauth.revoke_access_token(&access_token()).await.unwrap();
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
