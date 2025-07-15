use asknothingx2_util::api::{request::IntoRequestParts, Method, StatusCode};
use twitch_oauth_token::{ClientId, ClientSecret, RefreshRequest, RefreshToken, TokenUrl};
use url::Url;

mod server;

#[tokio::test]
async fn with_server() {
    let mock_uri = server::refresh().await;
    let client_id = ClientId::new("hof5gwx0su6owfnys0yan9c87zr6t".into());
    let client_secret = ClientSecret::new("".into());
    let refresh_token =
        RefreshToken::new("gdw3k62zpqi0kw01escg7zgbdhtxi6hm0155tiwcztxczkx17".into());
    let token_url = TokenUrl::new(format!("{mock_uri}/refresh")).unwrap();

    let request = RefreshRequest::new(&client_id, &client_secret, &refresh_token, &token_url)
        .into_request_parts()
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, request.status());
}

#[test]
fn request() {
    let client_id = ClientId::new("test_id".into());
    let client_secret = ClientSecret::new("test_secret".into());
    let refresh_token = RefreshToken::new("refres88efi".into());
    let token_url = TokenUrl::new("https://id.twitch.tv/oauth2/token".into()).unwrap();
    let request = RefreshRequest::new(&client_id, &client_secret, &refresh_token, &token_url);
    let params = vec![
        ("client_id", "test_id"),
        ("client_secret", "test_secret"),
        ("grant_type", "refresh_token"),
        ("refresh_token", "refres88efi"),
    ];

    let expected_body = url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params)
        .finish()
        .into_bytes();

    let request = request.into_request_parts();

    assert_eq!(Method::POST, request.method);
    assert_eq!(
        Url::parse("https://id.twitch.tv/oauth2/token").unwrap(),
        request.url
    );
    assert_eq!(2, request.headers.len());
    let expected_content_length = expected_body.len() as u64;
    let content_length = request.body.unwrap().content_length().unwrap();
    assert_eq!(expected_content_length, content_length);
}
