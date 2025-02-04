use std::collections::HashSet;

use asknothingx2_util::{
    api::{api_request, APIRequest, APIResponse, Method},
    oauth::{AuthUrl, ClientId, ClientSecret},
};
use url::Url;

use crate::{
    types::{scopes_mut, GrantType, Scope, ScopesMut},
    HttpError,
};

pub struct TestAccessToken {
    client_id: ClientId,
    client_secret: ClientSecret,
    grant_type: GrantType,
    user_id: Option<String>,
    scopes: HashSet<Scope>,
    auth_url: AuthUrl,
}

impl TestAccessToken {
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        grant_type: GrantType,
        user_id: Option<String>,
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

    pub async fn request_access_token(self) -> Result<APIResponse, HttpError> {
        let response = api_request(self).await?;

        Ok(APIResponse::from_response(response).await?)
    }
}

impl APIRequest for TestAccessToken {
    fn url(&self) -> Url {
        let mut url = self.auth_url.url().clone();

        let mut params = vec![
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.secret()),
            ("grant_type", self.grant_type.as_str()),
        ];

        let user_id = self.user_id.clone().unwrap_or_default();

        if !user_id.is_empty() {
            params.push(("user_id", &user_id));
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

        url
    }
    fn method(&self) -> Method {
        Method::POST
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use asknothingx2_util::{
        api::{APIRequest, Method},
        oauth::{AuthUrl, ClientId, ClientSecret},
    };
    use url::Url;

    use crate::types::{GrantType, Scope};

    use super::TestAccessToken;

    #[test]
    fn test_access_token() {
        let mut test_client = TestAccessToken {
            client_id: ClientId::new("ef74yew8ew".to_string()),
            client_secret: ClientSecret::new("fl790fiits".to_string()),
            grant_type: GrantType::UserToken,
            user_id: Some("8086138".to_string()),
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

        assert_eq!(0, test_client.headers().len());
        assert_eq!(Method::POST, test_client.method());
        assert_eq!(None, test_client.urlencoded());
    }
}
