use asknothingx2_http::Body;
use http::header::{ACCEPT, CONTENT_TYPE};

use crate::{
    AccessToken, ClientId,
    request::{APPLICATION_FORM_URLENCODED, APPLICATION_JSON, CLIENT_ID, IntoRequest, REVOKE_URL},
};

/// <https://dev.twitch.tv/docs/authentication/revoke-tokens/>
#[derive(Debug)]
pub struct RevokeRequest<'a> {
    access_token: &'a AccessToken,
    client_id: &'a ClientId,
}

impl<'a> RevokeRequest<'a> {
    pub fn new(access_token: &'a AccessToken, client_id: &'a ClientId) -> Self {
        Self {
            access_token,
            client_id,
        }
    }
}

impl IntoRequest for RevokeRequest<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::RevokeAccessToken;

    fn into_request(self) -> Result<asknothingx2_http::Request, crate::error::BoxError> {
        Ok(http::Request::post(REVOKE_URL)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_FORM_URLENCODED)
            .body(Body::from(super::form(&[
                (CLIENT_ID, self.client_id.as_str()),
                ("token", self.access_token.secret()),
            ])?))?)
    }
}
