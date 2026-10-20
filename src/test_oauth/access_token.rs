use std::{collections::HashSet, ops::Deref};

use asknothingx2_http::{Body, ClientService, client::Reqwest};
use asknothingx2_util::api::preset;

use crate::{
    AuthUrl, ClientId, ClientSecret,
    request::IntoRequest,
    scope::{Scope, ScopesMut, scopes_mut},
    test_oauth::USER_AGENT,
    types::GrantType,
};

#[derive(Debug, Clone)]
pub struct TestAccessToken<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    grant_type: GrantType,
    user_id: Option<&'a str>,
    scopes: HashSet<Scope>,
    auth_url: AuthUrl,
    svc: ClientService,
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
            svc: ClientService::new(Reqwest::new(preset::testing(USER_AGENT).build().unwrap())),
        }
    }

    pub fn scopes_mut(&mut self) -> ScopesMut<'_> {
        scopes_mut(&mut self.scopes)
    }

    pub async fn send(self) -> Result<crate::UserToken, crate::Error> {
        crate::client::json(self.svc.clone(), self).await
    }
}

impl IntoRequest for TestAccessToken<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::MockUserAccessToken;
    fn into_request(self) -> Result<asknothingx2_http::Request, crate::error::BoxError> {
        let mut url = self.auth_url.to_url();

        let mut params = vec![
            ("client_id", self.client_id.deref()),
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
        Ok(http::Request::post(url.to_string()).body(Body::empty())?)
    }
}
