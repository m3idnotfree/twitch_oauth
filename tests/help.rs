pub mod url {
    use twitch_oauth_token::RedirectUrl;
    use url::Url;

    use std::str::FromStr;

    pub fn redirect() -> RedirectUrl {
        RedirectUrl::from_str("http://localhost:3000").unwrap()
    }

    pub fn to_mock(mut url: Url, server: &wiremock::MockServer) -> Url {
        let mock = Url::parse(&server.uri()).unwrap();
        url.set_scheme(mock.scheme()).unwrap();
        url.set_host(mock.host_str()).unwrap();
        url.set_port(mock.port()).unwrap();
        url
    }
}

pub mod token {
    use twitch_oauth_token::{AccessToken, RefreshToken};

    pub fn refresh() -> RefreshToken {
        RefreshToken::from("eyJfaWQmNzMtNGCJ9%6VFV5LNrZFUj8oU231/3Aj")
    }

    pub fn access() -> AccessToken {
        AccessToken::from("jostpf5q0uzmxmkba9iyug38kjtgh")
    }

    pub fn code() -> String {
        "gulfwdmys5lsm6qyz4xiz9q32l10".to_string()
    }
}

#[cfg(feature = "test")]
pub mod setup {
    use asknothingx2_http::client::Reqwest;
    use twitch_oauth_token::{Client, UserAuth};
    use wiremock::MockServer;

    use super::service;
    use super::{config, url};

    pub async fn oauth_server() -> (MockServer, Client<UserAuth>) {
        let server = MockServer::start().await;

        let oauth = Client::builder(config::client_id().into(), config::client_secret().into())
            .http_client(service::Host::new(
                Reqwest::new(config::client()),
                &server.uri(),
            ))
            .build()
            .with_redirect_uri(url::redirect());

        (server, oauth)
    }
}

pub mod config {
    use asknothingx2_util::api::preset;
    use reqwest::Client;

    pub fn client_id() -> String {
        "hof5gwx0su6owfnys0yan9c87zr6t".to_string()
    }

    pub fn client_secret() -> String {
        "41vpdji4e9gif29md0ouet6fktd2".to_string()
    }

    pub fn client() -> Client {
        preset::testing("test/1.0").build().unwrap()
    }
}

pub mod validate {

    use super::config;

    pub fn form_client_id() -> String {
        format!("client_id={}", config::client_id())
    }

    pub fn form_client_secret() -> String {
        format!("client_secret={}", config::client_secret())
    }

    pub fn form_redirect() -> String {
        "redirect_uri=http%3A%2F%2Flocalhost%3A3000".to_string()
    }

    pub fn form_grant_type(kind: &str) -> String {
        format!("grant_type={kind}")
    }

    pub fn form_code(kind: &str) -> String {
        format!("code={kind}")
    }

    pub fn form_refresh_token() -> String {
        "refresh_token=eyJfaWQmNzMtNGCJ9%256VFV5LNrZFUj8oU231%2F3Aj".to_string()
    }
    pub fn form_token(token: &str) -> String {
        format!("token={token}")
    }
}

pub mod server {
    use serde_json::json;
    use twitch_oauth_token::{Client, UserAuth};
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_string_contains, header, method, path, query_param},
    };

    use super::{config, token, url, validate};

    pub async fn client_credentials(server: &MockServer) {
        Mock::given(method("POST"))
            .and(path("/oauth2/token"))
            .and(header("accept", "application/json"))
            .and(header("content-type", "application/x-www-form-urlencoded"))
            .and(body_string_contains(validate::form_client_id()))
            .and(body_string_contains(validate::form_client_secret()))
            .and(body_string_contains(validate::form_grant_type(
                "client_credentials",
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": token::access(),
                "expires_in": 5011271,
                "token_type": "bearer"
            })))
            .mount(server)
            .await;
    }

    pub async fn authorization_code_grant_flow(
        server: &MockServer,
        oauth: &Client<UserAuth>,
        scopes: &str,
    ) -> String {
        let auth_url = oauth.authorization_url().url();
        let state = auth_url
            .query_pairs()
            .find(|(k, _)| k == "state")
            .map(|(_, v)| v.to_string())
            .unwrap();

        Mock::given(method("GET"))
            .and(path("/oauth2/authorize"))
            .and(query_param("client_id", config::client_id()))
            .and(query_param("redirect_uri", url::redirect().to_string()))
            .and(query_param("response_type", "code"))
            .and(query_param("state", state.clone()))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": token::code(),
                "scope": scopes,
                "state": state
            })))
            .mount(server)
            .await;

        Mock::given(method("POST"))
            .and(path("/oauth2/token"))
            .and(header("accept", "application/json"))
            .and(header("content-type", "application/x-www-form-urlencoded"))
            .and(body_string_contains(validate::form_client_id()))
            .and(body_string_contains(validate::form_client_secret()))
            .and(body_string_contains(validate::form_code(&token::code())))
            .and(body_string_contains(validate::form_grant_type(
                "authorization_code",
            )))
            .and(body_string_contains(validate::form_redirect()))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": token::access(),
                "expires_in": 14124,
                "refresh_token": token::refresh(),
                "scope": [
                    "channel:moderate",
                    "chat:edit",
                    "chat:read"
                ],
                "token_type": "bearer"
            })))
            .mount(server)
            .await;

        state
    }

    pub async fn refresh_access_token(server: &MockServer) {
        Mock::given(method("POST"))
            .and(path("/oauth2/token"))
            .and(header("accept", "application/json"))
            .and(header("content-type", "application/x-www-form-urlencoded"))
            .and(body_string_contains(validate::form_client_id()))
            .and(body_string_contains(validate::form_client_secret()))
            .and(body_string_contains(validate::form_grant_type(
                "refresh_token",
            )))
            .and(body_string_contains(validate::form_refresh_token()))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": token::access(),
                "expires_in": 14124,
                "refresh_token": token::refresh(),
                "scope": [
                    "channel:moderate",
                    "chat:edit",
                    "chat:read"
                ],
                "token_type": "bearer"
            })))
            .mount(server)
            .await;
    }

    pub async fn validate_access_token(server: &MockServer) {
        Mock::given(method("GET"))
            .and(path("oauth2/validate"))
            .and(header(
                "Authorization",
                format!("OAuth {}", token::access().secret()),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
              "client_id": config::client_id(),
              "login": "twitchdev",
              "scopes": [
                "channel:read:subscriptions"
              ],
              "user_id": "141981764",
              "expires_in": 5520838
            })))
            .mount(server)
            .await;
    }

    pub async fn revoke_access_token(server: &MockServer) {
        Mock::given(method("POST"))
            .and(path("/oauth2/revoke"))
            .and(header("accept", "application/json"))
            .and(header("content-type", "application/x-www-form-urlencoded"))
            .and(body_string_contains(validate::form_client_id()))
            .and(body_string_contains(validate::form_token(
                token::access().secret(),
            )))
            .respond_with(ResponseTemplate::new(204))
            .mount(server)
            .await;
    }
}

pub mod service {
    use asknothingx2_http::Request;
    use http::Uri;
    use tower_service::Service;

    #[derive(Clone)]
    pub struct Host<S> {
        inner: S,
        base: Uri,
    }

    impl<S> Host<S> {
        pub fn new(inner: S, base: &str) -> Self {
            Self {
                inner,
                base: base.parse().unwrap(),
            }
        }
    }

    impl<S> Service<Request> for Host<S>
    where
        S: Service<Request>,
    {
        type Response = S::Response;
        type Error = S::Error;
        type Future = S::Future;

        fn poll_ready(
            &mut self,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx)
        }

        fn call(&mut self, mut req: Request) -> Self::Future {
            let mut parts = req.uri().clone().into_parts();
            let base = self.base.clone().into_parts();
            parts.scheme = base.scheme;
            parts.authority = base.authority;
            *req.uri_mut() = Uri::from_parts(parts).unwrap();

            self.inner.call(req)
        }
    }
}
