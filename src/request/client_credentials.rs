use std::ops::Deref;

use asknothingx2_http::Body;
use http::header::{ACCEPT, CONTENT_TYPE};

use crate::{
    ClientId, ClientSecret,
    request::{
        APPLICATION_FORM_URLENCODED, APPLICATION_JSON, CLIENT_ID, CLIENT_SECRET, GRANT_TYPE,
        IntoRequest, TOKEN_URL,
    },
    types::GrantType,
};

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
#[derive(Debug)]
pub struct ClientCredentialsRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
}

impl<'a> ClientCredentialsRequest<'a> {
    pub fn new(client_id: &'a ClientId, client_secret: &'a ClientSecret) -> Self {
        Self {
            client_id,
            client_secret,
        }
    }
}

impl IntoRequest for ClientCredentialsRequest<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::AppAccessToken;

    fn into_request(self) -> Result<asknothingx2_http::Request, crate::error::BoxError> {
        Ok(http::Request::post(TOKEN_URL)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_FORM_URLENCODED)
            .body(Body::from(super::form(&[
                (CLIENT_ID, self.client_id.deref()),
                (CLIENT_SECRET, self.client_secret.secret()),
                (GRANT_TYPE, GrantType::ClientCredentials.as_str()),
            ])?))?)
    }
}
