use std::collections::HashSet;

use asknothingx2_util::{
    api::{preset, IntoRequestBuilder, Method},
    oauth::{AuthUrl, ClientId, ClientSecret},
};
use reqwest::Response;

use crate::{
    types::{scopes_mut, GrantType, Scope, ScopesMut},
    Error,
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
        let client = preset::testing("test/1.0").build_client().unwrap();
        self.into_request_builder(&client)
            .unwrap()
            .send()
            .await
            .map_err(asknothingx2_util::api::Error::from)
    }
}

impl IntoRequestBuilder for TestAccessToken<'_> {
    type Error = Error;
    fn into_request_builder(
        self,
        client: &reqwest::Client,
    ) -> Result<reqwest::RequestBuilder, Self::Error> {
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
        Ok(client.request(Method::POST, url))
    }
}
