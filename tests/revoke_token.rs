use asknothingx2_util::api::{request::IntoRequestParts, Method, StatusCode};
use twitch_oauth_token::{AccessToken, ClientId, RevocationUrl, RevokeRequest};
use url::Url;

mod server;

#[tokio::test]
async fn with_server() {
    let mock_uri = server::revoke().await;

    let access_toen = AccessToken::new("rfx2uswqe8l4g1mkagrvg5tv0ks3".into());
    let client_id = ClientId::new("hof5gwx0su6owfnys0yan9c87zr6t".into());
    let revocation_url = RevocationUrl::new(format!("{mock_uri}/revoke")).unwrap();

    let a = RevokeRequest::new(&access_toen, &client_id, &revocation_url)
        .into_request_parts()
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, a.status());
}

#[test]
fn request() {
    let access_token = AccessToken::new("ue85uei4ui".into());
    let client_id = ClientId::new("test_id".into());
    let revocatin_url = RevocationUrl::new("https://id.twitch.tv/oauth2/revoke".into()).unwrap();

    let request = RevokeRequest::new(&access_token, &client_id, &revocatin_url);

    let params = vec![("client_id", "test_id"), ("token", "ue85uei4ui")];

    let expected_body = url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params)
        .finish()
        .into_bytes();

    let request = request.into_request_parts();

    assert_eq!(Method::POST, request.method);
    assert_eq!(
        Url::parse("https://id.twitch.tv/oauth2/revoke").unwrap(),
        request.url
    );
    assert_eq!(2, request.headers.len());
    let expected_content_length = expected_body.len() as u64;
    let content_length = request.body.unwrap().content_length().unwrap();
    assert_eq!(expected_content_length, content_length);
}
