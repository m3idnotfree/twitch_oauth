use std::collections::HashSet;

use crate::{types::GrantType, TwitchOauth};

mod get_users_info;
mod user_access_token;

pub use get_users_info::{get_users_info, User, Users};
pub use user_access_token::TestAccessToken;

pub trait TwitchTest {
    fn with_url<T: Into<String>>(&mut self, url: T) -> &mut Self;
    fn user_token<T: Into<String>>(&self, user_id: T) -> TestAccessToken;
    fn app_token(&self) -> TestAccessToken;
}

#[derive(Debug, Default)]
pub struct TestUrlHold(Option<String>);

impl TestUrlHold {
    pub fn with_url<T: Into<String>>(&mut self, url: T) -> &mut Self {
        self.0 = Some(url.into());
        self
    }

    pub fn get_test_url(&self) -> Option<url::Url> {
        self.0.as_ref().map(|url| url::Url::parse(url).unwrap())
    }
}

impl TwitchTest for TwitchOauth {
    fn with_url<T: Into<String>>(&mut self, url: T) -> &mut Self {
        self.test_url.with_url(url.into());
        self
    }

    /// Getting a user access token
    fn user_token<T: Into<String>>(&self, user_id: T) -> TestAccessToken {
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
    fn app_token(&self) -> TestAccessToken {
        TestAccessToken::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            GrantType::ClientCredentials,
            "",
            HashSet::new(),
            self.get_auth_url(),
        )
    }
}
