mod get_users_info;
mod user_access_token;

pub use get_users_info::{get_users_info, User, UsersResponse};
pub use user_access_token::TestAccessToken;

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
