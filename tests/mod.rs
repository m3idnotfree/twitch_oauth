use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

pub async fn server() -> String {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/hello"))
        .respond_with(ResponseTemplate::new(200))
        // Mounting the mock on the mock server - it's now effective!
        .mount(&mock_server)
        .await;

    mock_server.uri()
}
