use std::str::FromStr;

use twitch_oauth_token::{
    AuthorizationCode, Client, ClientId, ClientSecret, ErrorKind, RedirectUri,
};

#[tokio::test]
async fn csrf_validation_failure() {
    let oauth = Client::new(
        ClientId::new("client_id".to_string()),
        ClientSecret::new("client_secret".to_string()),
    )
    .with_redirect_uri(RedirectUri::from_str("http://localhost:3000").unwrap());

    let result = oauth
        .exchange_code(&AuthorizationCode::new("code".to_string()), "state")
        .await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(matches!(error.kind(), ErrorKind::Csrf));
}
