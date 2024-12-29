mod server;

use asknothingx2_util::{
    api::{api_request, APIRequest, HeaderBuilder, StatusCode},
    oauth::{AccessToken, ValidateUrl},
};
use reqwest::Method;
use twitch_oauth_token::ValidateRequest;
use url::Url;

#[tokio::test]
async fn with_server() {
    let mock_uri = server::validate("rfx2uswqe8l4g1mkagrvg5tv0ks3").await;

    let a = api_request(ValidateRequest::new(
        AccessToken::new("rfx2uswqe8l4g1mkagrvg5tv0ks3".to_string()),
        ValidateUrl::new(format!("{}/validate", mock_uri)).unwrap(),
    ))
    .await
    .unwrap();

    assert_eq!(StatusCode::OK, a.status());
}

#[test]
fn request() {
    let request = ValidateRequest::new(
        AccessToken::new("ue85uei4ui".to_string()),
        ValidateUrl::new("https://id.twitch.tv/oauth2/validate".to_string()).unwrap(),
    );

    let expected_headers = HeaderBuilder::new()
        .authorization("OAuth", "ue85uei4ui")
        .build();

    assert_eq!(Method::GET, request.method());
    assert_eq!(
        Url::parse("https://id.twitch.tv/oauth2/validate").unwrap(),
        request.url()
    );
    assert_eq!(expected_headers, request.headers());
}
