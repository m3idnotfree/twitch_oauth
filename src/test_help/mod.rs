use std::collections::HashSet;

use crate::{types::GrantType, TwitchOauth};

mod get_users_info;
pub use get_users_info::*;
mod user_access_token;
pub use user_access_token::*;

pub trait TwitchTest {
    fn with_url<T: Into<String>>(&mut self, url: T) -> &mut Self;
    fn get_mock_user_access_token<T: Into<String>>(&self, user_id: T) -> TestAccessToken;
    fn get_mock_app_access_token(&self) -> TestAccessToken;
}

impl TwitchTest for TwitchOauth {
    fn with_url<T: Into<String>>(&mut self, url: T) -> &mut Self {
        self.test_url = Some(url.into());
        self
    }

    fn get_mock_user_access_token<T: Into<String>>(&self, user_id: T) -> TestAccessToken {
        TestAccessToken::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            GrantType::UserToken,
            user_id.into(),
            HashSet::new(),
            self.get_auth_url(),
        )
    }

    fn get_mock_app_access_token(&self) -> TestAccessToken {
        TestAccessToken::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            GrantType::ClientCredentials,
            "".into(),
            HashSet::new(),
            self.get_auth_url(),
        )
    }
}
