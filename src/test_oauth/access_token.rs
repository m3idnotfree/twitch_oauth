use std::{collections::HashSet, ops::Deref};

use asknothingx2_util::api::{preset, IntoRequestBuilder, Method};
use reqwest::Client;

use crate::{
    error,
    response::UserTokenResponse,
    scope::{scopes_mut, Scope, ScopesMut},
    types::GrantType,
    AuthUrl, ClientId, ClientSecret, Error,
};

#[derive(Debug, Clone)]
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

    pub async fn send(self) -> Result<crate::response::Response<UserTokenResponse>, crate::Error> {
        let client = preset::testing("twitch-oauth-test/1.0")
            .build_client()
            .unwrap();
        send(self, &client).await
    }
}

impl IntoRequestBuilder for TestAccessToken<'_> {
    type Error = Error;
    fn into_request_builder(
        self,
        client: &reqwest::Client,
    ) -> Result<reqwest::RequestBuilder, Self::Error> {
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
        Ok(client.request(Method::POST, url))
    }
}

pub async fn send<T, R>(
    request: T,
    client: &Client,
) -> Result<crate::response::Response<R>, T::Error>
where
    T: IntoRequestBuilder<Error = Error>,
    R: crate::response::ResponseType,
{
    let resp = request
        .into_request_builder(client)?
        .send()
        .await
        .map_err(error::network::request)?;

    if !resp.status().is_success() {
        let status = resp.status();
        match resp.text().await {
            Ok(body) => {
                return Err(Error::with_message(
                    error::Kind::OAuthError,
                    format!("HTTP {status}: {body}"),
                ));
            }
            Err(e) => {
                return Err(Error::with_message(
                    error::Kind::OAuthError,
                    format!("HTTP {status} - Failed to read error response: {e}"),
                ))
            }
        }
    }

    Ok(crate::response::Response::new(resp))
}
