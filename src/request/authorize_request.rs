use std::collections::HashSet;

use url::Url;

use crate::{
    ClientId, RedirectUri, State,
    request::{AUTH_URL, CLIENT_ID},
    scope::{Scope, ScopesMut, scopes_mut},
    types::ResponseType,
};

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
#[derive(Debug)]
pub struct AuthorizationRequest<'a> {
    client_id: &'a ClientId,
    force_verify: Option<bool>,
    redirect_url: &'a RedirectUri,
    scopes: HashSet<Scope>,
}

impl<'a> AuthorizationRequest<'a> {
    pub(crate) fn new(client_id: &'a ClientId, redirect_url: &'a RedirectUri) -> Self {
        Self {
            client_id,
            force_verify: None,
            redirect_url,
            scopes: HashSet::new(),
        }
    }

    pub fn scopes_mut(&mut self) -> ScopesMut<'_> {
        scopes_mut(&mut self.scopes)
    }

    pub fn set_force_verify(&mut self, force_verify: bool) -> &mut Self {
        self.force_verify = Some(force_verify);
        self
    }

    /// Returns the authorization URL and the [`State`]
    pub fn url(self) -> (Url, State) {
        let mut url: Url = Url::parse(AUTH_URL).expect("AUTH_URL is a valid URL");
        let state = State::generate();

        {
            let mut query_pairs = url.query_pairs_mut();

            query_pairs.extend_pairs([
                (CLIENT_ID, self.client_id.as_str()),
                ("redirect_uri", self.redirect_url.as_str()),
                ("response_type", ResponseType::Code.as_str()),
                ("state", state.secret()),
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

        (url, state)
    }
}
