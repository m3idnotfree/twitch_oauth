use asknothingx2_util::{
    api::{APIRequest, Method},
    oauth::{ClientId, ClientSecret, TokenUrl},
};
use twitch_oauth_token::{types::GrantType, ClientCredentialsRequest};
use url::Url;

#[test]
fn request() {
    let request = ClientCredentialsRequest::new(
        ClientId::new("test_id".to_string()),
        ClientSecret::new("test_secret".to_string()),
        GrantType::ClientCredentials,
        TokenUrl::new("https://id.twitch.tv/oauth2/token".to_string()).unwrap(),
    );

    let params = vec![
        ("client_id", "test_id"),
        ("client_secret", "test_secret"),
        ("grant_type", "client_credentials"),
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
