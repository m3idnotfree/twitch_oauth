use std::ops::Deref;

use asknothingx2_http::Body;
use asknothingx2_util::api::mime_type::Application;
use http::header::{ACCEPT, CONTENT_TYPE};

use crate::{
    AccessToken, ClientId,
    request::{CLIENT_ID, IntoRequest, REVOKE_URL},
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
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(Body::from(super::form(&[
                (CLIENT_ID, self.client_id.deref()),
                ("token", self.access_token.secret()),
            ])?))?)
    }
}
