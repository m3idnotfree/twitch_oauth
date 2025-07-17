use std::collections::HashSet;

use asknothingx2_util::oauth::{AuthUrl, ClientId, RedirectUrl};
use url::Url;

use crate::types::{scopes_mut, ResponseType, Scope, ScopesMut};

use super::CLIENT_ID;

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
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

    pub fn url(self) -> Url {
        let mut url: Url = self.auth_url.url().to_owned();

        {
            let mut query_pairs = url.query_pairs_mut();

            query_pairs.extend_pairs([
                (CLIENT_ID, self.client_id.as_str()),
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
}
