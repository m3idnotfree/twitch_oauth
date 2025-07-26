mod access_token;
mod get_users_info;

use std::collections::HashSet;

pub use access_token::TestAccessToken;
use asknothingx2_util::oauth::{AuthUrl, ClientId, ClientSecret};
pub use get_users_info::{get_users_info, User, UsersResponse};
use url::Url;

use crate::{oauth::OauthFlow, types::GrantType, Error, TwitchOauth};

pub trait OauthTestExt<State: OauthFlow> {
    fn with_test_env(self, env: TestEnv) -> TwitchOauthTest<State>;
}

impl<State> OauthTestExt<State> for TwitchOauth<State>
where
    State: OauthFlow,
{
    fn with_test_env(self, env: TestEnv) -> TwitchOauthTest<State> {
        TwitchOauthTest {
            oauth: self,
            test_env: env,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TwitchOauthTest<State>
where
    State: OauthFlow,
{
    oauth: TwitchOauth<State>,
    test_env: TestEnv,
}

impl<State> TwitchOauthTest<State>
where
    State: OauthFlow,
{
    pub fn oauth(&self) -> &TwitchOauth<State> {
        &self.oauth
    }

    pub fn create_user_token<'a>(&'a self, user_id: &'a str) -> TestAccessToken<'a> {
        self.test_env
            .user_token(self.oauth.client_id(), self.oauth.client_secret(), user_id)
    }

    pub fn create_app_token(&self) -> TestAccessToken {
        self.test_env
            .app_token(self.oauth.client_id(), self.oauth.client_secret())
    }

    pub async fn get_users_info(&self) -> Result<UsersResponse, Error> {
        get_users_info(self.test_env.port).await
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
        Self {
            port: None,
            user_auth_url: None,
            app_auth_url: None,
        }
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
            Some(Url::parse(&format!(
                "http://localhost:{port}/auth/authorize"
            ))?)
        };

        let app_auth_url = if self.app_auth_url.is_some() {
            self.app_auth_url
        } else {
            Some(Url::parse(&format!("http://localhost:{port}/auth/token"))?)
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

    pub fn user_token<'a>(
        &'a self,
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        user_id: &'a str,
    ) -> TestAccessToken<'a> {
        TestAccessToken::new(
            client_id,
            client_secret,
            GrantType::UserToken,
            Some(user_id),
            HashSet::new(),
            AuthUrl::new(self.user_auth_url().to_string()).unwrap(),
        )
    }

    pub fn app_token<'a>(
        &'a self,
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
    ) -> TestAccessToken<'a> {
        TestAccessToken::new(
            client_id,
            client_secret,
            GrantType::ClientCredentials,
            None::<&str>,
            HashSet::new(),
            AuthUrl::new(self.app_auth_url().to_string()).unwrap(),
        )
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
