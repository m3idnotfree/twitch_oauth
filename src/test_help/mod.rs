use std::collections::HashSet;

use crate::{types::GrantType, TwitchOauth};

mod get_users_info;
pub use get_users_info::{get_users_info, User, Users};
mod user_access_token;
pub use user_access_token::TestAccessToken;

pub trait TwitchTest {
    fn with_url<T: Into<String>>(&mut self, url: T);
    fn get_mock_user_access_token<T: Into<String>>(&self, user_id: T) -> TestAccessToken;
    fn get_mock_app_access_token(&self) -> TestAccessToken;
}

impl TwitchTest for TwitchOauth {
    fn with_url<T: Into<String>>(&mut self, url: T) {
        self.test_url = Some(url.into());
    }

    /// Getting a user access token
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

    /// Getting an app access token
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
