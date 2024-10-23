use asknothingx2_util::{
    api::{APIRequest, Method},
    oauth::{AuthUrl, ClientId, ClientSecret},
};
use url::Url;

use crate::{
    scopes::{self, Scopes, ScopesMut},
    types::GrantType,
};

pub struct TestAccessToken<'a> {
    pub client_id: &'a ClientId,
    pub client_secret: &'a ClientSecret,
    pub grant_type: GrantType,
    pub user_id: String,
    pub scopes: Vec<Scopes>,
    pub auth_url: &'a AuthUrl,
}

impl TestAccessToken<'_> {
    pub fn scopes_mut(&mut self) -> ScopesMut<'_> {
        scopes::new(&mut self.scopes)
    }

    pub fn set_user_id(mut self, user_id: &str) -> Self {
        self.user_id = user_id.to_string();
        self
    }
    // pub fn add_scope(mut self, scope: Scopes) -> Self {
    //     self.scopes.add_scope(scope);
    //     self
    // }
    //
    // pub fn add_scopes<I>(mut self, scopes: I) -> Self
    // where
    //     I: IntoIterator<Item = Scopes>,
    // {
    //     self.scopes.add_scopes(scopes);
    //     self
    // }
}

impl APIRequest for TestAccessToken<'_> {
    fn url(&self) -> Url {
        let mut auth_url = self.auth_url.url().clone();

        let scopes = self
            .scopes
            .clone()
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join(" ");

        let mut params = vec![
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.secret()),
            ("grant_type", self.grant_type.as_ref()),
            ("user_id", self.user_id.as_ref()),
        ];

        if !scopes.is_empty() {
            params.push(("scope", &scopes));
        }

        auth_url.query_pairs_mut().extend_pairs(params);

        auth_url
    }

    fn method(&self) -> Method {
        Method::POST
    }
}

#[cfg(test)]
mod test {
    use asknothingx2_util::{
        api::{APIRequest, Method},
        oauth::{AuthUrl, ClientId, ClientSecret},
    };
    use url::Url;

    use crate::{scopes::Scopes, types::GrantType};

    use super::TestAccessToken;

    #[test]
    fn test_access_token() {
        let mut test_client = TestAccessToken {
            client_id: &ClientId::new("ef74yew8ew".to_string()),
            client_secret: &ClientSecret::new("fl790fiits".to_string()),
            grant_type: GrantType::UserToken,
            user_id: "8086138".to_string(),
            scopes: Vec::new(),
            auth_url: &AuthUrl::new("http://localhost:8080/auth/authorize".to_string()).unwrap(),
        };

        test_client
            .scopes_mut()
            .push(Scopes::UserBot)
            .extend([Scopes::ChannelBot, Scopes::UserWriteChat]);

        // let test_client = test_client.add_scope(Scopes::UserBot);
        // let test_client = test_client.add_scopes([Scopes::ChannelBot, Scopes::UserWriteChat]);

        let mut expected_auth_url = Url::parse("http://localhost:8080/auth/authorize").unwrap();

        let expected_params = vec![
            ("client_id", "ef74yew8ew"),
            ("client_secret", "fl790fiits"),
            ("grant_type", "user_token"),
            ("user_id", "8086138"),
            ("scope", "user:bot channel:bot user:write:chat"),
        ];

        expected_auth_url
            .query_pairs_mut()
            .extend_pairs(expected_params);

        assert_eq!(0, test_client.headers().len());
        assert_eq!(Method::POST, test_client.method());
        assert_eq!(expected_auth_url, test_client.url());
        assert_eq!(None, test_client.urlencoded());
    }
}
