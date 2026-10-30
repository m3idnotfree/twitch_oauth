use std::ops::Deref;

use asknothingx2_http::Body;
use http::header::{ACCEPT, CONTENT_TYPE};

use crate::{
    ClientId, ClientSecret, RefreshToken,
    request::{
        APPLICATION_FORM_URLENCODED, APPLICATION_JSON, CLIENT_ID, CLIENT_SECRET, GRANT_TYPE,
        IntoRequest, TOKEN_URL,
    },
    types::GrantType,
};

/// <https://dev.twitch.tv/docs/authentication/refresh-tokens/>
#[derive(Debug)]
pub struct RefreshRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    refresh_token: &'a RefreshToken,
}

impl<'a> RefreshRequest<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        refresh_token: &'a RefreshToken,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            refresh_token,
        }
    }
}

impl IntoRequest for RefreshRequest<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::RefreshAccessToken;

    fn into_request(self) -> Result<asknothingx2_http::Request, crate::error::BoxError> {
        Ok(http::Request::post(TOKEN_URL)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_FORM_URLENCODED)
            .body(Body::from(super::form(&[
                (CLIENT_ID, self.client_id.deref()),
                (CLIENT_SECRET, self.client_secret.secret()),
                (GRANT_TYPE, GrantType::RefreshToken.as_str()),
                ("refresh_token", self.refresh_token.secret()),
            ])?))?)
    }
}
