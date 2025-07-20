use std::collections::HashSet;

use asknothingx2_util::api::{
    preset,
    request::{IntoRequestParts, RequestParts},
    Method,
};
use reqwest::Response;

use crate::{
    types::{scopes_mut, GrantType, Scope, ScopesMut},
    AuthUrl, ClientId, ClientSecret,
};

pub struct TestAccessToken<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    grant_type: GrantType,
    user_id: Option<&'a str>,
    scopes: HashSet<Scope>,
    auth_url: AuthUrl,
}

impl<'a> TestAccessToken<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        grant_type: GrantType,
        user_id: Option<&'a str>,
        scopes: HashSet<Scope>,
        auth_url: AuthUrl,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            grant_type,
            user_id,
            scopes,
            auth_url,
        }
    }

    pub fn scopes_mut(&mut self) -> ScopesMut<'_> {
        scopes_mut(&mut self.scopes)
    }

    pub async fn request_access_token(self) -> Result<Response, asknothingx2_util::api::Error> {
        let client = preset::for_test("test/1.0").build_client().unwrap();
        self.into_request_parts()
            .into_request_builder(&client)
            .unwrap()
            .send()
            .await
            .map_err(asknothingx2_util::api::Error::from)
    }
}

impl IntoRequestParts for TestAccessToken<'_> {
    fn into_request_parts(self) -> RequestParts {
        let mut url = self.auth_url.url().clone();

        let mut params = vec![
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.secret()),
            ("grant_type", self.grant_type.as_str()),
        ];

        let user_id = self.user_id.unwrap_or_default();

        if !user_id.is_empty() {
            params.push(("user_id", user_id));
        }

        let scopes = self
            .scopes
            .clone()
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join(" ");

        if !scopes.is_empty() {
            params.push(("scope", &scopes));
        }

        url.query_pairs_mut().extend_pairs(params);

        RequestParts::new(Method::POST, url)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use asknothingx2_util::api::{request::IntoRequestParts, Method};
    use url::Url;

    use crate::{
        types::{GrantType, Scope},
        AuthUrl, ClientId, ClientSecret,
    };

    use super::TestAccessToken;

    #[test]
    fn test_access_token() {
        let mut test_client = TestAccessToken {
            client_id: &ClientId::new("ef74yew8ew".to_string()),
            client_secret: &ClientSecret::new("fl790fiits".to_string()),
            grant_type: GrantType::UserToken,
            user_id: Some("8086138"),
            scopes: HashSet::new(),
            auth_url: AuthUrl::new("http://localhost:8080/auth/authorize".to_string()).unwrap(),
        };

        test_client
            .scopes_mut()
            .push(Scope::UserBot)
            .extend([Scope::ChannelBot, Scope::UserWriteChat]);

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

        let test_client = test_client.into_request_parts();

        assert_eq!(0, test_client.headers.len());
        assert_eq!(Method::POST, test_client.method);
        assert!(test_client.body.is_none());
    }
}
