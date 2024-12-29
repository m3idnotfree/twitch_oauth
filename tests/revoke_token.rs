use asknothingx2_util::{
    api::{api_request, APIRequest, StatusCode},
    oauth::{AccessToken, ClientId, RevocationUrl},
};
use reqwest::Method;
use twitch_oauth_token::RevokeRequest;
use url::Url;

mod server;

#[tokio::test]
async fn with_server() {
    let mock_uri = server::revoke().await;

    let a = api_request(RevokeRequest::new(
        AccessToken::new("rfx2uswqe8l4g1mkagrvg5tv0ks3".to_string()),
        ClientId::new("hof5gwx0su6owfnys0yan9c87zr6t".to_string()),
        RevocationUrl::new(format!("{}/revoke", mock_uri)).unwrap(),
    ))
    .await
    .unwrap();

    assert_eq!(StatusCode::OK, a.status());
}

#[test]
fn request() {
    let request = RevokeRequest::new(
        AccessToken::new("ue85uei4ui".to_string()),
        ClientId::new("test_id".to_string()),
        RevocationUrl::new("https://id.twitch.tv/oauth2/revoke".to_string()).unwrap(),
    );

    let params = vec![("client_id", "test_id"), ("token", "ue85uei4ui")];

    let expected_body = url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params)
        .finish()
        .into_bytes();

    assert_eq!(Method::POST, request.method());
    assert_eq!(
        Url::parse("https://id.twitch.tv/oauth2/revoke").unwrap(),
        request.url()
    );
    assert_eq!(2, request.headers().len());
    assert_eq!(Some(expected_body), request.urlencoded());
}
