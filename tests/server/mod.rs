#![allow(dead_code)]
use wiremock::{
    matchers::{body_bytes, header, method, path},
    Mock, MockServer, ResponseTemplate,
};

pub async fn revoke() -> String {
    let params = vec![
        ("client_id", "hof5gwx0su6owfnys0yan9c87zr6t"),
        ("token", "rfx2uswqe8l4g1mkagrvg5tv0ks3"),
    ];
    let body = url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params)
        .finish()
        .into_bytes();

    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/revoke"))
        .and(body_bytes(body))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    mock_server.uri()
}
pub async fn validate(access_token: &str) -> String {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/validate"))
        .and(header("Authorization", format!("OAuth {}", access_token)))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    mock_server.uri()
}

pub async fn refresh() -> String {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/refresh"))
        .and(header("Content-Type", "application/x-www-form-urlencoded"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    mock_server.uri()
}

// use wiremock::{
//     matchers::{method, path},
//     Mock, MockServer, ResponseTemplate,
// };
//
// pub async fn server() -> String {
//     let mock_server = MockServer::start().await;
//
//     Mock::given(method("GET"))
//         .and(path("/hello"))
//         .respond_with(ResponseTemplate::new(200))
//         // Mounting the mock on the mock server - it's now effective!
//         .mount(&mock_server)
//         .await;
//
//     mock_server.uri()
// }
