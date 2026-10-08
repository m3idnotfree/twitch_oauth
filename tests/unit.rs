use std::str::FromStr;

use twitch_oauth_token::{AuthorizationCode, ErrorKind, RedirectUrl, TwitchOauth};

#[tokio::test]
async fn csrf_validation_failure() {
    let oauth = TwitchOauth::new("client_id", "client_secret")
        .with_redirect_uri(RedirectUrl::from_str("http://localhost:3000").unwrap());

    let result = oauth
        .exchange_code(AuthorizationCode::from("code"), "state".to_string())
        .await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(matches!(error.kind(), ErrorKind::Csrf));
}
