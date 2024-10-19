mod server;

use http::StatusCode;
use oauth2::AccessToken;
use twitch_oauth_token::request::{oauth_request, ValidateRequest, ValidateUrl};

#[tokio::test]
async fn validate_token() {
    let mock_uri = server::validate().await;

    let a = oauth_request(ValidateRequest {
        access_token: &AccessToken::new("rfx2uswqe8l4g1mkagrvg5tv0ks3".to_string()),
        validate_url: &ValidateUrl::new(format!("{}/validate", mock_uri)).unwrap(),
    })
    .await
    .unwrap();

    assert_eq!(StatusCode::OK, a.status);
}
