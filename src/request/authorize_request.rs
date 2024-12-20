use std::collections::HashSet;

use asknothingx2_util::oauth::{AuthUrl, ClientId, CsrfToken, RedirectUrl};
use url::Url;

use crate::{
    scopes::{self, Scopes, ScopesMut},
    types::ResponseType,
};

pub struct AuthrozationRequest {
    auth_url: AuthUrl,
    client_id: ClientId,
    redirect_url: RedirectUrl,
    response_type: ResponseType,
    scopes: HashSet<Scopes>,
    state: CsrfToken,
}

impl AuthrozationRequest {
    pub fn new(
        auth_url: AuthUrl,
        client_id: ClientId,
        redirect_url: RedirectUrl,
        response_type: ResponseType,
        state: CsrfToken,
    ) -> Self {
        Self {
            auth_url,
            client_id,
            redirect_url,
            response_type,
            scopes: HashSet::new(),
            state,
        }
    }

    pub fn scopes_mut(&mut self) -> ScopesMut<'_> {
        scopes::new(&mut self.scopes)
    }

    pub fn url(self) -> Url {
        let scopes = self
            .scopes
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join(" ");

        let url = {
            let mut pairs = vec![
                ("client_id", self.client_id.as_str()),
                ("redirect_uri", self.redirect_url.as_str()),
                ("response_type", self.response_type.as_ref()),
                ("state", self.state.secret()),
            ];

            if !scopes.is_empty() {
                pairs.push(("scope", &scopes));
            }

            let mut url: Url = self.auth_url.url().to_owned();

            url.query_pairs_mut().extend_pairs(pairs);

            url
        };

        url
    }
}
