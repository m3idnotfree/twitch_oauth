use std::future::Future;

use asknothingx2_util::oauth::AuthUrl;
use test_access_token::TestAccessToken;
use url::Url;

use crate::{types::GrantType, TwitchOauth};

mod get_users_info;
mod test_access_token;
pub use get_users_info::*;

pub trait TwitchTest {
    fn test_init(self, port: Option<u16>) -> Self;
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
