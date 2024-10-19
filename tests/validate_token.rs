mod server;

use http::StatusCode;
use oauth2::AccessToken;
use twitch_oauth_token::request::{oauth_request, ValidateRequest, ValidateUrl};

#[tokio::test]
async fn validate_token() {
    let mock_uri = server::validate().await;

    let a = oauth_request(ValidateRequest {
        // client_id: &ClientId::new("hof5gwx0su6owfnys0yan9c87zr6t".to_string()),
        // revoke_url: &RevocationUrl::new(format!("{}/revoke", mock_uri)).unwrap(),
        access_token: &AccessToken::new("rfx2uswqe8l4g1mkagrvg5tv0ks3".to_string()),
        validate_url: &ValidateUrl::new(format!("{}/validate", mock_uri)).unwrap(),
    })
    .await
    .unwrap();

    assert_eq!(StatusCode::OK, a.status);

    // let mut client = TwitchOauth::default()
    //     .set_client_id("hof5gwx0su6owfnys0yan9c87zr6t")
    //     .set_client_secret("41vpdji4e9gif29md0ouet6fktd2");
}
