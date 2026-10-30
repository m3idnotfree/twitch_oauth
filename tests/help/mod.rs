use asknothingx2_http::{Request, config::preset};
use http::{HeaderValue, Uri};
use tower_layer::Layer;
use tower_service::Service;
use url::Url;

pub const ACCESS_TOKEN: &str = "jostpf5q0uzmxmkba9iyug38kjtgh";
pub const CLIENT_ID: &str = "hof5gwx0su6owfnys0yan9c87zr6t";
pub const CLIENT_SECRET: &str = "41vpdji4e9gif29md0ouet6fktd2";
pub const CODE: &str = "gulfwdmys5lsm6qyz4xiz9q32l10";
pub const DEVICE_CODE: &str = "ike3GM8QIdYZs43KdrWPIO36LofILoCyFEzjlQ91";
pub const REDIRECT_URL: &str = "http://localhost:3000";
pub const REFRESH_TOKEN: &str = "eyJfaWQmNzMtNGCJ9%6VFV5LNrZFUj8oU231/3Aj";

pub fn client() -> reqwest::Client {
    preset::danger_test(
        reqwest::Client::builder(),
        HeaderValue::from_static(concat!(
            env!("CARGO_PKG_NAME"),
            "-test/",
            env!("CARGO_PKG_VERSION")
        )),
    )
    .build()
    .unwrap()
}

pub mod setup {
    use std::str::FromStr;

    use twitch_oauth_token::{Client, ClientId, DeviceAuth, RedirectUrl, UserAuth};
    use wiremock::MockServer;

    use super::{CLIENT_ID, CLIENT_SECRET, OriginLayer, REDIRECT_URL};

    pub async fn oauth_server() -> (MockServer, Client<UserAuth>) {
        let server = MockServer::start().await;

        let oauth = Client::builder(CLIENT_ID.into(), CLIENT_SECRET.into())
            .layer(OriginLayer::new(&server.uri()))
            .http_client(super::client())
            .with_redirect_uri(RedirectUrl::from_str(REDIRECT_URL).unwrap());

        (server, oauth)
    }

    pub async fn device_server() -> (MockServer, DeviceAuth) {
        let server = MockServer::start().await;

        let oauth = DeviceAuth::builder(ClientId::from(CLIENT_ID))
            .layer(OriginLayer::new(&server.uri()))
            .http_client(super::client());

        (server, oauth)
    }
}

pub mod server {
    use std::collections::HashSet;

    use serde_json::json;
    use twitch_oauth_token::Scope;
    use url::Url;
    use wiremock::{
        Mock, MockBuilder, MockServer, ResponseTemplate,
        matchers::{BodyContainsMatcher, body_string_contains, header, method, path, query_param},
    };

    use super::{
        ACCESS_TOKEN, CLIENT_ID, CLIENT_SECRET, CODE, DEVICE_CODE, REDIRECT_URL, REFRESH_TOKEN,
    };

    fn form(key: &str, value: &str) -> BodyContainsMatcher {
        body_string_contains(serde_urlencoded::to_string([(key, value)]).unwrap())
    }

    fn post(p: &str) -> MockBuilder {
        Mock::given(method("POST"))
            .and(path(p))
            .and(header("accept", "application/json"))
            .and(header("content-type", "application/x-www-form-urlencoded"))
    }

    fn user_token() -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(json!({
            "access_token": ACCESS_TOKEN,
            "expires_in": 15485,
            "refresh_token": REFRESH_TOKEN,
            "scope": [
                "channel:moderate",
                "chat:edit",
                "chat:read"
            ],
            "token_type": "bearer"
        }))
    }

    pub async fn client_credentials(server: &MockServer) {
        post("/oauth2/token")
            .and(form("client_id", CLIENT_ID))
            .and(form("client_secret", CLIENT_SECRET))
            .and(form("grant_type", "client_credentials"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": ACCESS_TOKEN,
                "expires_in": 5011271,
                "token_type": "bearer"
            })))
            .mount(server)
            .await;
    }

    pub async fn authorization_code_grant_flow(
        server: &MockServer,
        url: &Url,
        scopes: &[Scope],
    ) -> String {
        let state = url
            .query_pairs()
            .find(|(k, _)| k == "state")
            .map(|(_, v)| v.to_string())
            .unwrap();

        let expected: HashSet<Scope> = scopes.iter().copied().collect();
        let has_scopes = move |req: &wiremock::Request| {
            req.url
                .query_pairs()
                .find(|(k, _)| k == "scope")
                .is_some_and(|(_, v)| {
                    v.split(' ')
                        .map(|s| s.parse().unwrap())
                        .collect::<HashSet<Scope>>()
                        == expected
                })
        };

        Mock::given(method("GET"))
            .and(path("/oauth2/authorize"))
            .and(query_param("client_id", CLIENT_ID))
            .and(query_param("redirect_uri", REDIRECT_URL))
            .and(query_param("response_type", "code"))
            .and(query_param("state", state.clone()))
            .and(has_scopes)
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": CODE,
                "scope": scopes.iter().map(AsRef::as_ref).collect::<Vec<&str>>().join(" "),
                "state": state
            })))
            .mount(server)
            .await;

        post("/oauth2/token")
            .and(form("client_id", CLIENT_ID))
            .and(form("client_secret", CLIENT_SECRET))
            .and(form("code", CODE))
            .and(form("grant_type", "authorization_code"))
            .and(form("redirect_uri", REDIRECT_URL))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": ACCESS_TOKEN,
                "expires_in": 15485,
                "refresh_token": REFRESH_TOKEN,
                "scope": scopes,
                "token_type": "bearer"
            })))
            .mount(server)
            .await;

        state
    }

    pub async fn refresh_access_token(server: &MockServer) {
        post("/oauth2/token")
            .and(form("client_id", CLIENT_ID))
            .and(form("client_secret", CLIENT_SECRET))
            .and(form("grant_type", "refresh_token"))
            .and(form("refresh_token", REFRESH_TOKEN))
            .respond_with(user_token())
            .mount(server)
            .await;
    }

    pub async fn validate_access_token(server: &MockServer) {
        Mock::given(method("GET"))
            .and(path("/oauth2/validate"))
            .and(header("Authorization", format!("OAuth {ACCESS_TOKEN}")))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "client_id": CLIENT_ID,
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
        post("/oauth2/revoke")
            .and(form("client_id", CLIENT_ID))
            .and(form("token", ACCESS_TOKEN))
            .respond_with(ResponseTemplate::new(204))
            .mount(server)
            .await;
    }

    pub async fn device_code_grant_flow(server: &MockServer) {
        post("/oauth2/device")
            .and(form("client_id", CLIENT_ID))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "device_code": DEVICE_CODE,
                "expires_in": 1800,
                "interval": 0,
                "user_code": "ABCDEFGH",
                "verification_uri": "https://www.twitch.tv/activate?public=true&device-code=ABCDEFGH"
            })))
            .mount(server)
            .await;

        let token_request = || {
            post("/oauth2/token")
                .and(form("client_id", CLIENT_ID))
                .and(form("device_code", DEVICE_CODE))
                .and(form(
                    "grant_type",
                    "urn:ietf:params:oauth:grant-type:device_code",
                ))
        };

        token_request()
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "status": 400,
                "message": "authorization_pending"
            })))
            .up_to_n_times(1)
            .with_priority(1)
            .mount(server)
            .await;

        token_request()
            .respond_with(user_token())
            .mount(server)
            .await;
    }
}

pub fn swap_origin(mut url: Url, server: &wiremock::MockServer) -> Url {
    let mock = Url::parse(&server.uri()).unwrap();
    url.set_scheme(mock.scheme()).unwrap();
    url.set_host(mock.host_str()).unwrap();
    url.set_port(mock.port()).unwrap();
    url
}

#[derive(Clone)]
pub struct OriginLayer {
    base: Uri,
}

impl OriginLayer {
    pub fn new(base: &str) -> Self {
        Self {
            base: base.parse().unwrap(),
        }
    }
}

impl<S> Layer<S> for OriginLayer {
    type Service = Origin<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Origin {
            inner,
            base: self.base.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Origin<S> {
    inner: S,
    base: Uri,
}

impl<S> Service<Request> for Origin<S>
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
