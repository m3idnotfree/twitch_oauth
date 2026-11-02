use std::collections::HashSet;

use asknothingx2_http::{Body, ClientService, Request};
use http::header::ACCEPT;
use url::Url;

use crate::{
    Client, ClientId, ClientSecret, Error, UserToken,
    client::{OauthFlow, json},
    request::{APPLICATION_JSON, AUTH_URL, CLIENT_ID, CLIENT_SECRET, GRANT_TYPE, IntoRequest},
    scope::{Scope, ScopesMut, scopes_mut},
    types::GrantType,
};

impl<Flow> Client<Flow>
where
    Flow: OauthFlow,
{
    /// Get a user token from the mock API
    ///
    /// Needs [`MockApiService`](super::MockApiService)
    #[allow(deprecated)]
    pub fn mock_user_token<'a>(&'a self, user_id: &'a str) -> MockUserTokenRequest<'a> {
        MockUserTokenRequest {
            client_id: self.client_id(),
            client_secret: self.client_secret(),
            user_id,
            scopes: HashSet::new(),
            svc: self.svc.clone(),
        }
    }
}

/// <https://dev.twitch.tv/docs/cli/mock-api-command/#getting-an-access-token>
#[derive(Debug)]
pub struct MockUserTokenRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    user_id: &'a str,
    scopes: HashSet<Scope>,
    svc: ClientService,
}

impl MockUserTokenRequest<'_> {
    pub fn scopes_mut(&mut self) -> ScopesMut<'_> {
        scopes_mut(&mut self.scopes)
    }

    pub async fn send(self) -> Result<UserToken, Error> {
        json(self.svc.clone(), self).await
    }
}

impl IntoRequest for MockUserTokenRequest<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::MockUserToken;

    fn into_request(self) -> Result<Request, crate::error::BoxError> {
        let scopes = self
            .scopes
            .iter()
            .map(Scope::as_str)
            .collect::<Vec<_>>()
            .join(" ");

        let mut url = Url::parse(AUTH_URL)?;
        url.query_pairs_mut()
            .append_pair(CLIENT_ID, self.client_id.as_str())
            .append_pair(CLIENT_SECRET, self.client_secret.secret())
            .append_pair(GRANT_TYPE, GrantType::MockUserToken.as_str())
            .append_pair("user_id", self.user_id)
            .append_pair("scope", &scopes);

        Ok(http::Request::post(url.as_str())
            .header(ACCEPT, APPLICATION_JSON)
            .body(Body::empty())?)
    }
}
