mod server;

use http::StatusCode;
use oauth2::{AccessToken, ClientId, RevocationUrl};
use twitch_oauth_token::request::{oauth_request, RevokeRequest};

#[tokio::test]
async fn revoke_token() {
    let mock_uri = server::revoke().await;

    let a = oauth_request(RevokeRequest {
        client_id: &ClientId::new("hof5gwx0su6owfnys0yan9c87zr6t".to_string()),
        revoke_url: &RevocationUrl::new(format!("{}/revoke", mock_uri)).unwrap(),
        access_token: &AccessToken::new("rfx2uswqe8l4g1mkagrvg5tv0ks3".to_string()),
    })
    .await
    .unwrap();

    assert_eq!(StatusCode::OK, a.status);
}
