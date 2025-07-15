use asknothingx2_util::api::{request::IntoRequestParts, Method};
use twitch_oauth_token::{ClientCredentialsRequest, ClientId, ClientSecret};
use url::Url;

#[test]
fn request() {
    let client_id = ClientId::new("test_id".into());
    let client_secret = ClientSecret::new("test_secret".into());
    let request = ClientCredentialsRequest::new(&client_id, &client_secret);

    let params = vec![
        ("client_id", "test_id"),
        ("client_secret", "test_secret"),
        ("grant_type", "client_credentials"),
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
    let expected_cntent_length = expected_body.len() as u64;
    let content_length = request.body.unwrap().content_length().unwrap();
    assert_eq!(expected_cntent_length, content_length);
}
