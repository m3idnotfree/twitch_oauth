mod server;

use asknothingx2_util::{
    api::{api_request, StatusCode},
    oauth::{AccessToken, ClientId, RevocationUrl},
};
use twitch_oauth_token::request::RevokeRequest;

#[tokio::test]
async fn revoke_token() {
    let mock_uri = server::revoke().await;

    let a = api_request(RevokeRequest {
        client_id: &ClientId::new("hof5gwx0su6owfnys0yan9c87zr6t".to_string()),
        revoke_url: &RevocationUrl::new(format!("{}/revoke", mock_uri)).unwrap(),
        access_token: &AccessToken::new("rfx2uswqe8l4g1mkagrvg5tv0ks3".to_string()),
    })
    .await
    .unwrap();

    assert_eq!(StatusCode::OK, a.status());
}
