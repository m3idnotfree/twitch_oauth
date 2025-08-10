pub mod mock_api;
pub mod response;

mod access_token;

pub use access_token::TestAccessToken;

use std::collections::HashSet;

use asknothingx2_util::{
    api::preset,
    oauth::{AuthUrl, TokenUrl},
};
use url::Url;

use crate::{
    oauth::OauthFlow,
    request::ClientCredentialsRequest,
    response::{AppTokenResponse, Response},
    types::GrantType,
    Error, TwitchOauth,
};

pub trait OauthTestExt<Flow: OauthFlow> {
    fn with_test_env(self, env: TestEnv) -> TwitchOauthTest<Flow>;
}

impl<Flow> OauthTestExt<Flow> for TwitchOauth<Flow>
where
    Flow: OauthFlow,
{
    fn with_test_env(self, env: TestEnv) -> TwitchOauthTest<Flow> {
        let this = self.set_client(
            preset::testing("twitch-oauth-token-test/1.0")
                .build_client()
                .unwrap(),
        );
        TwitchOauthTest {
            oauth: this,
            test_env: env,
        }
    }
}

#[derive(Debug)]
pub struct TwitchOauthTest<Flow>
where
    Flow: OauthFlow,
{
    oauth: TwitchOauth<Flow>,
    test_env: TestEnv,
}

impl<Flow> TwitchOauthTest<Flow>
where
    Flow: OauthFlow,
{
    pub fn oauth(&self) -> &TwitchOauth<Flow> {
        &self.oauth
    }

    pub fn user_access_token<'a>(&'a self, user_id: &'a str) -> TestAccessToken<'a> {
        TestAccessToken::new(
            self.oauth.client_id(),
            self.oauth.client_secret(),
            GrantType::UserToken,
            Some(user_id),
            HashSet::new(),
            AuthUrl::new(self.test_env.user_auth_url().to_string()).unwrap(),
        )
    }

    pub async fn app_access_token(&self) -> Result<Response<AppTokenResponse>, crate::Error> {
        self.oauth
            .send(ClientCredentialsRequest::new(
                self.oauth.client_id(),
                self.oauth.client_secret(),
                GrantType::ClientCredentials,
                &TokenUrl::new(self.test_env.app_auth_url().to_string()).unwrap(),
            ))
            .await
    }
}

#[derive(Debug, Clone)]
pub struct TestEnv {
    port: Option<u16>,
    user_auth_url: Option<Url>,
    app_auth_url: Option<Url>,
}

impl TestEnv {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn with_user_auth_url(mut self, url: Url) -> Self {
        self.user_auth_url = Some(url);
        self
    }

    pub fn with_app_auth_url(mut self, url: Url) -> Self {
        self.app_auth_url = Some(url);
        self
    }

    pub fn configure(self) -> Result<TestEnv, Error> {
        let port = self.port.unwrap_or(8080);
        let user_auth_url = if self.user_auth_url.is_some() {
            self.user_auth_url
        } else {
            Some(Url::parse(&format!("http://localhost:{port}/auth/authorize")).unwrap())
        };

        let app_auth_url = if self.app_auth_url.is_some() {
            self.app_auth_url
        } else {
            Some(Url::parse(&format!("http://localhost:{port}/auth/token")).unwrap())
        };

        Ok(TestEnv {
            port: Some(port),
            user_auth_url,
            app_auth_url,
        })
    }

    pub fn user_auth_url(&self) -> &Url {
        self.user_auth_url
            .as_ref()
            .expect("URLs should be configured")
    }

    pub fn app_auth_url(&self) -> &Url {
        self.app_auth_url
            .as_ref()
            .expect("URLs should be configured")
    }

    pub fn port(&self) -> u16 {
        self.port.expect("Port should be configured")
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        let user_auth_url = Some(Url::parse("http://localhost:8080/auth/authorize").unwrap());
        let app_auth_url = Some(Url::parse("http://localhost:8080/auth/token").unwrap());

        TestEnv {
            port: Some(8080),
            user_auth_url,
            app_auth_url,
        }
    }
}
