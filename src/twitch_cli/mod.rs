use std::future::Future;

use asknothingx2_util::oauth::AuthUrl;
use serde::{Deserialize, Serialize};
use test_access_token::TestAccessToken;
use url::Url;

use crate::{types::GrantType, TwitchOauth};

mod test_access_token;

pub trait TwitchTest {
    fn test_init(self, port: Option<u16>) -> Self;
    fn get_mock_users_info(&self) -> impl Future<Output = Result<Users, reqwest::Error>> + Send;
    fn get_mock_access_token(&self, user_id: &str) -> TestAccessToken;
}

impl TwitchTest for TwitchOauth {
    fn test_init(self, port: Option<u16>) -> Self {
        let mut base_url = Url::parse("http://localhost:8080").unwrap();
        if let Some(port) = port {
            base_url.set_port(Some(port)).unwrap();
        }

        let mut auth_url = base_url.clone();
        auth_url
            .path_segments_mut()
            .unwrap()
            .extend(["auth", "authorize"]);

        self.set_base_url(base_url)
            .set_auth_url(AuthUrl::new(auth_url.to_string()).unwrap())
    }

    async fn get_mock_users_info(&self) -> Result<Users, reqwest::Error> {
        let mut units_clients = self.base_url.clone();
        units_clients
            .path_segments_mut()
            .unwrap()
            .push("units")
            .push("clients");

        reqwest::get(units_clients).await?.json().await
    }

    fn get_mock_access_token(&self, user_id: &str) -> TestAccessToken {
        TestAccessToken {
            client_id: &self.client_id,
            client_secret: &self.client_secret,
            grant_type: GrantType::UserToken,
            user_id: user_id.to_string(),
            scopes: Vec::new(),
            auth_url: &self.auth_url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub cursor: String,
    pub total: u64,
    pub data: Vec<User>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub ID: String,
    pub Secret: String,
    pub Name: String,
    pub IsExtension: bool,
}
