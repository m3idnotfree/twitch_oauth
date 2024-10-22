use asknothingx2_util::{
    api::APIRequest,
    oauth::{AuthUrl, ClientId, ClientSecret},
};
use url::Url;

use crate::{
    request::POST_formencoded_header,
    scopes::{ScopeBuilder, Scopes},
    types::GrantType,
};

pub struct TestAccessToken<'a> {
    pub client_id: &'a ClientId,
    pub client_secret: &'a ClientSecret,
    pub grant_type: GrantType,
    pub user_id: String,
    pub scopes: ScopeBuilder,
    pub auth_url: &'a AuthUrl,
}

impl TestAccessToken<'_> {
    pub fn set_user_id(mut self, user_id: &str) -> Self {
        self.user_id = user_id.to_string();
        self
    }
    pub fn add_scope(mut self, scope: Scopes) -> Self {
        self.scopes.add_scope(scope);
        self
    }

    pub fn add_scopes<I>(mut self, scopes: I) -> Self
    where
        I: IntoIterator<Item = Scopes>,
    {
        self.scopes.add_scopes(scopes);
        self
    }
}

impl APIRequest for TestAccessToken<'_> {
    fn url(&self) -> Url {
        let mut auth_url = self.auth_url.url().clone();

        let scopes = self.scopes.clone().build();

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

    fn headers(&self) -> http::HeaderMap {
        POST_formencoded_header()
    }

    fn method(&self) -> http::Method {
        http::Method::POST
    }
}
