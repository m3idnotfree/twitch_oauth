mod server;

use asknothingx2_util::{
    api::{api_request, StatusCode},
    oauth::{AccessToken, ValidateUrl},
};
use twitch_oauth_token::request::ValidateRequest;

#[tokio::test]
async fn validate_token() {
    let mock_uri = server::validate("rfx2uswqe8l4g1mkagrvg5tv0ks3").await;

    let a = api_request(ValidateRequest {
        access_token: &AccessToken::new("rfx2uswqe8l4g1mkagrvg5tv0ks3".to_string()),
        validate_url: &ValidateUrl::new(format!("{}/validate", mock_uri)).unwrap(),
    })
    .await
    .unwrap();

    assert_eq!(StatusCode::OK, a.status());
}
