use std::collections::HashSet;

use asknothingx2_util::oauth::AuthUrl;
use url::Url;

use crate::{types::GrantType, TwitchOauth};

mod get_users_info;
pub use get_users_info::*;
mod test_access_token;
pub use test_access_token::*;

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
        TestAccessToken::new(
            &self.client_id,
            &self.client_secret,
            GrantType::UserToken,
            user_id.to_string(),
            HashSet::new(),
            &self.auth_url,
        )
    }
}
