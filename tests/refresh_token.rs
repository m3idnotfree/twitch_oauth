use asknothingx2_util::{
    api::{request::IntoRequestParts, Method, StatusCode},
    oauth::{ClientId, ClientSecret, RefreshToken, TokenUrl},
};
use twitch_oauth_token::{types::GrantType, RefreshRequest};
use url::Url;

mod server;

#[tokio::test]
async fn with_server() {
    let mock_uri = server::refresh().await;

    let request = RefreshRequest::new(
        ClientId::new("hof5gwx0su6owfnys0yan9c87zr6t".to_string()),
        ClientSecret::new("".to_string()),
        GrantType::RefreshToken,
        RefreshToken::new("gdw3k62zpqi0kw01escg7zgbdhtxi6hm0155tiwcztxczkx17".to_string()),
        TokenUrl::new(format!("{mock_uri}/refresh")).unwrap(),
    )
    .into_request_parts()
    .send()
    .await
    .unwrap();

    assert_eq!(StatusCode::OK, request.status());
}

#[test]
fn request() {
    let request = RefreshRequest::new(
        ClientId::new("test_id".to_string()),
        ClientSecret::new("test_secret".to_string()),
        GrantType::RefreshToken,
        RefreshToken::new("refres88efi".to_string()),
        TokenUrl::new("https://id.twitch.tv/oauth2/token".to_string()).unwrap(),
    );
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
