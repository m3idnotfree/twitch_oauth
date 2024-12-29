use std::collections::HashSet;

use asknothingx2_util::oauth::{AuthUrl, ClientId, CsrfToken, RedirectUrl};
use url::Url;

use crate::types::{scopes_mut, ResponseType, Scope, ScopesMut};

/// https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow
pub struct AuthrozationRequest {
    auth_url: AuthUrl,
    client_id: ClientId,
    force_verify: Option<bool>,
    redirect_url: RedirectUrl,
    response_type: ResponseType,
    scopes: HashSet<Scope>,
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
            force_verify: None,
            redirect_url,
            response_type,
            scopes: HashSet::new(),
            state,
        }
    }

    pub fn scopes_mut(&mut self) -> ScopesMut<'_> {
        scopes_mut(&mut self.scopes)
    }

    pub fn set_force_verify(&mut self, force_verify: bool) -> &mut Self {
        self.force_verify = Some(force_verify);
        self
    }

    pub fn url(self) -> Url {
        let mut pairs = vec![
            ("client_id", self.client_id.as_str()),
            ("redirect_uri", self.redirect_url.as_str()),
            ("response_type", self.response_type.as_str()),
            ("state", self.state.secret()),
        ];

        let force_verify = if let Some(verify) = self.force_verify {
            verify.to_string()
        } else {
            String::new()
        };

        let scopes = self
            .scopes
            .into_iter()
            .map(|x| x.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        if !scopes.is_empty() {
            pairs.push(("scope", &scopes));
        }

        if !force_verify.is_empty() {
            pairs.push(("force_verify", &force_verify));
        }

        let mut url: Url = self.auth_url.url().clone();

        url.query_pairs_mut().extend_pairs(pairs);

        url
    }
}
