use asknothingx2_util::{
    api::{request::IntoRequestParts, AuthScheme, HeaderMut, Method, StatusCode},
    oauth::{AccessToken, ValidateUrl},
};
use twitch_oauth_token::ValidateRequest;
use url::Url;
use wiremock::http::HeaderMap;

mod server;

#[tokio::test]
async fn with_server() {
    let mock_uri = server::validate("rfx2uswqe8l4g1mkagrvg5tv0ks3").await;

    let request = ValidateRequest::new(
        AccessToken::new("rfx2uswqe8l4g1mkagrvg5tv0ks3".to_string()),
        ValidateUrl::new(format!("{mock_uri}/validate")).unwrap(),
    )
    .into_request_parts()
    .send()
    .await
    .unwrap();

    assert_eq!(StatusCode::OK, request.status());
}

#[test]
fn request() {
    let request = ValidateRequest::new(
        AccessToken::new("ue85uei4ui".to_string()),
        ValidateUrl::new("https://id.twitch.tv/oauth2/validate".to_string()).unwrap(),
    );

    let mut expected_headers = HeaderMap::new();
    HeaderMut::new(&mut expected_headers).authorization(AuthScheme::custom("OAuth", "ue85uei4ui"));

    let request = request.into_request_parts();

    assert_eq!(Method::GET, request.method);
    assert_eq!(
        Url::parse("https://id.twitch.tv/oauth2/validate").unwrap(),
        request.url
    );
    assert_eq!(expected_headers, request.headers);
}
