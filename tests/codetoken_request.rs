use asknothingx2_util::api::{request::IntoRequestParts, Method};
use twitch_oauth_token::{
    AuthorizationCode, ClientId, ClientSecret, CodeTokenRequest, RedirectUrl,
};
use url::Url;

#[test]
fn request() {
    let client_id = ClientId::new("test_id".into());
    let client_secret = ClientSecret::new("test_secret".into());
    let authorization_code = AuthorizationCode::new("authorization_code".into());
    let redirect_url = RedirectUrl::new("http://localhost:3000".into()).unwrap();

    let request = CodeTokenRequest::new(
        &client_id,
        &client_secret,
        &authorization_code,
        &redirect_url,
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
