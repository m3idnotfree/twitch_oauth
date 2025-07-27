pub mod url {
    use twitch_oauth_token::{AuthUrl, RedirectUrl, RevocationUrl, TokenUrl, ValidateUrl};

    pub fn token(url: &str) -> TokenUrl {
        TokenUrl::new(format!("{url}/oauth2/token",)).unwrap()
    }

    pub fn auth(url: &str) -> AuthUrl {
        AuthUrl::new(format!("{url}/oauth2/authorize")).unwrap()
    }

    pub fn redirect() -> RedirectUrl {
        RedirectUrl::new("http://localhost:3000".to_string()).unwrap()
    }

    pub fn revoke(url: &str) -> RevocationUrl {
        RevocationUrl::new(format!("{url}/oauth2/revoke")).unwrap()
    }

    pub fn validate(url: &str) -> ValidateUrl {
        ValidateUrl::new(format!("{url}/oauth2/validate")).unwrap()
    }
}

pub mod token {
    use twitch_oauth_token::{AccessToken, RefreshToken};

    pub fn refresh() -> RefreshToken {
        RefreshToken::new("eyJfaWQmNzMtNGCJ9%6VFV5LNrZFUj8oU231/3Aj".to_string())
    }

    pub fn access() -> AccessToken {
        AccessToken::new("jostpf5q0uzmxmkba9iyug38kjtgh".to_string())
    }

    pub fn code() -> String {
        "gulfwdmys5lsm6qyz4xiz9q32l10".to_string()
    }
}

pub mod setup {
    use twitch_oauth_token::{TwitchOauth, UserAuth};
    use wiremock::MockServer;

    use super::{config, url};

    pub async fn oauth_server() -> (MockServer, TwitchOauth<UserAuth>) {
        let server = MockServer::start().await;

        let oauth = TwitchOauth::new(config::client_id(), config::client_secret())
            .set_client(config::client())
            .set_token_url(url::token(&server.uri()))
            .set_auth_url(url::auth(&server.uri()))
            .set_validate_url(url::validate(&server.uri()))
            .set_revoke_url(url::revoke(&server.uri()))
            .set_redirect_uri(url::redirect());

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
        preset::testing("test/1.0").build_client().unwrap()
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
    use twitch_oauth_token::{TwitchOauth, UserAuth};
    use wiremock::{
        matchers::{body_string_contains, header, method, path, query_param},
        Mock, MockServer, ResponseTemplate,
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
        oauth: &TwitchOauth<UserAuth>,
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
