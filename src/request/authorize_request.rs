use std::{collections::HashSet, ops::Deref};

use url::Url;

use crate::{
    scope::{scopes_mut, Scope, ScopesMut},
    types::ResponseType,
    AuthUrl, ClientId, RedirectUrl,
};

use super::CLIENT_ID;

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
#[derive(Debug)]
pub struct AuthrozationRequest<'a> {
    auth_url: &'a AuthUrl,
    client_id: &'a ClientId,
    force_verify: Option<bool>,
    redirect_url: &'a RedirectUrl,
    scopes: HashSet<Scope>,
    state: String,
}

impl<'a> AuthrozationRequest<'a> {
    pub fn new(
        auth_url: &'a AuthUrl,
        client_id: &'a ClientId,
        redirect_url: &'a RedirectUrl,
        state: String,
    ) -> Self {
        Self {
            auth_url,
            client_id,
            force_verify: None,
            redirect_url,
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

    /// Generate authorization URL (stateless CSRF protection)
    pub fn url(self) -> Url {
        let mut url: Url = self.auth_url.to_url();

        {
            let mut query_pairs = url.query_pairs_mut();

            query_pairs.extend_pairs([
                (CLIENT_ID, self.client_id.deref()),
                ("redirect_uri", self.redirect_url),
                ("response_type", ResponseType::Code.as_str()),
                ("state", &self.state),
            ]);

            let scopes = self
                .scopes
                .into_iter()
                .map(|x| x.as_str())
                .collect::<Vec<_>>()
                .join(" ");

            query_pairs.append_pair("scope", &scopes);

            if let Some(force_verify) = self.force_verify {
                query_pairs.append_pair("force_verify", &force_verify.to_string());
            }
        }

        url
    }

    /// Generate authorization URL with state
    pub fn url_with_state(self) -> (Url, String) {
        let state = self.state.clone();
        let url = self.url();

        (url, state)
    }
}
