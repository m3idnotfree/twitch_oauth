use asknothingx2_util::api::{preset, AuthScheme, HeaderMut, Method, StatusCode};
use twitch_oauth_token::{AccessToken, IntoRequestBuilder, ValidateRequest, ValidateUrl};
use url::Url;
use wiremock::http::HeaderMap;

mod server;

#[tokio::test]
async fn with_server() {
    let mock_uri = server::validate("rfx2uswqe8l4g1mkagrvg5tv0ks3").await;

    let access_token = AccessToken::new("rfx2uswqe8l4g1mkagrvg5tv0ks3".into());
    let validate_url = ValidateUrl::new(format!("{mock_uri}/validate")).unwrap();
    let client = preset::for_test("test/1.0").build_client().unwrap();
    let request = ValidateRequest::new(&access_token, &validate_url)
        .into_request_builder(&client)
        .unwrap()
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, request.status());
}

#[test]
fn request() {
    let access_token = AccessToken::new("ue85uei4ui".into());
    let validate_url = ValidateUrl::new("https://id.twitch.tv/oauth2/validate".into()).unwrap();
    let request = ValidateRequest::new(&access_token, &validate_url);
    let client = preset::for_test("test/1.0").build_client().unwrap();

    let mut expected_headers = HeaderMap::new();
    HeaderMut::new(&mut expected_headers).authorization(AuthScheme::custom("OAuth", "ue85uei4ui"));

    let request = request
        .into_request_builder(&client)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(Method::GET, request.method());
    assert_eq!(
        &Url::parse("https://id.twitch.tv/oauth2/validate").unwrap(),
        request.url()
    );
    assert_eq!(&expected_headers, request.headers());
}
