use std::ops::Deref;

use asknothingx2_http::Body;
use asknothingx2_util::api::mime_type::Application;
use http::header::{ACCEPT, CONTENT_TYPE};

use crate::{
    ClientId, ClientSecret, TokenUrl,
    request::{CLIENT_ID, CLIENT_SECRET, GRANT_TYPE, IntoRequest},
    types::GrantType,
};

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
#[derive(Debug)]
pub struct ClientCredentialsRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    grant_type: GrantType,
    token_url: &'a TokenUrl,
}

impl<'a> ClientCredentialsRequest<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        grant_type: GrantType,
        token_url: &'a TokenUrl,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            grant_type,
            token_url,
        }
    }
}

impl IntoRequest for ClientCredentialsRequest<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::AppAccessToken;

    fn into_request(self) -> Result<asknothingx2_http::Request, crate::error::BoxError> {
        Ok(http::Request::post(self.token_url.to_string())
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(Body::from(super::form(&[
                (CLIENT_ID, self.client_id.deref()),
                (CLIENT_SECRET, self.client_secret.secret()),
                (GRANT_TYPE, self.grant_type.as_str()),
            ])?))?)
    }
}
