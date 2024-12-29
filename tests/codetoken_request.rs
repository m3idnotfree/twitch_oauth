use asknothingx2_util::{
    api::{APIRequest, Method},
    oauth::{AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenUrl},
};
use twitch_oauth_token::{types::GrantType, CodeTokenRequest};
use url::Url;

#[test]
fn request() {
    let request = CodeTokenRequest::new(
        ClientId::new("test_id".to_string()),
        ClientSecret::new("test_secret".to_string()),
        AuthorizationCode::new("authorization_code".to_string()),
        GrantType::AuthorizationCode,
        TokenUrl::new("https://id.twitch.tv/oauth2/token".to_string()).unwrap(),
        RedirectUrl::new("http://localhost:3000".to_string()).unwrap(),
    );
    let params = vec![
        ("client_id", "test_id"),
        ("client_secret", "test_secret"),
        ("code", "authorization_code"),
        ("grant_type", "authorization_code"),
        ("redirect_uri", "http://localhost:3000"),
    ];

    let expected_body = url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params)
        .finish()
        .into_bytes();

    assert_eq!(Method::POST, request.method());
    assert_eq!(
        Url::parse("https://id.twitch.tv/oauth2/token").unwrap(),
        request.url()
    );
    assert_eq!(2, request.headers().len());
    assert_eq!(Some(expected_body), request.urlencoded());
}
