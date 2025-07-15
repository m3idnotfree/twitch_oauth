use std::borrow::Cow;
use std::collections::HashSet;

use asknothingx2_util::oauth::{AuthUrl, ClientId, RedirectUrl};
use url::Url;

use crate::{
    error,
    types::{scopes_mut, ResponseType, Scope, ScopesMut},
    Error,
};

use super::CLIENT_ID;

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
pub struct AuthrozationRequest<'a> {
    auth_url: &'a AuthUrl,
    client_id: &'a ClientId,
    force_verify: Option<bool>,
    redirect_url: Option<Cow<'a, RedirectUrl>>,
    scopes: HashSet<Scope>,
    state: String,
}

impl<'a> AuthrozationRequest<'a> {
    pub fn new(
        auth_url: &'a AuthUrl,
        client_id: &'a ClientId,
        redirect_url: Option<&'a RedirectUrl>,
        state: String,
    ) -> Self {
        Self {
            auth_url,
            client_id,
            force_verify: None,
            redirect_url: redirect_url.map(Cow::Borrowed),
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

    pub fn url(self) -> Result<Url, Error> {
        let redirect_url = self
            .redirect_url
            .ok_or_else(error::oauth::missing_redirect_url)?;

        let mut pairs = vec![
            (CLIENT_ID, self.client_id.as_str()),
            ("redirect_uri", redirect_url.as_ref()),
            ("response_type", ResponseType::Code.as_str()),
            ("state", &self.state),
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

        let mut url: Url = self.auth_url.url().to_owned();

        url.query_pairs_mut().extend_pairs(pairs);

        Ok(url)
    }
}
